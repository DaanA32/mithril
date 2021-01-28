#![allow(unknown_lints)]
#![allow(inline_always)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use crate::u64x2::u64x2;
use crate::u64x2::u32x4;

unsafe fn _mm_shuffle_epi32(key: u64x2, order: u16) -> u64x2 {
    let sections: u32x4 = std::mem::transmute(key);
    let i0 = order & 0b00000011; 
    let i1 = order & 0b00001100 >> 2;
    let i2 = order & 0b00110000 >> 4;
    let i3 = order & 0b11000000 >> 6;
    let result = u32x4(sections[i0], sections[i1], sections[i2], sections[i3]);
    std::mem::transmute(result)
}

macro_rules! mm_shuffle_epi32 {
    ($key:expr, $ib:expr, $result:ident) => {
        // llvm_asm!(concat!("pshufd xmm1, xmm2, ", $ib)
        //     : "={xmm1}"($result)
        //     : "{xmm2}"($key)
        //     :
        //     : "intel", "alignstack", "volatile"
        // );
        $result = _mm_shuffle_epi32($key, $ib);
    }
}

#[inline(always)]
pub fn _mm_shuffle_epi32_0x55(key: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_shuffle_epi32!(key, 0x55, r)
    }
    r
}

#[inline(always)]
pub fn _mm_shuffle_epi32_0xff(key: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_shuffle_epi32!(key, 0xFF, r)
    }
    r
}

#[inline(always)]
pub fn _mm_shuffle_epi32_0xaa(key: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_shuffle_epi32!(key, 0xAA, r)
    }
    r
}

macro_rules! mm_cvtsi128_si32 {
    ($v:expr, $result:ident) => {
        // llvm_asm!("movd eax, xmm1"
        //     : "={eax}"($result)
        //     : "{xmm1}"($v)
        //     :
        //     : "intel", "alignstack", "volatile"
        // );
        // asm!(
        //     "st1.4s {{v0}}, x0",
        //     in("v0") $v,
        //     out("x0") $result,
        // )
    }
}

#[inline(always)]
pub fn _mm_cvtsi128_si32(v: u64x2) -> u32 {
    unsafe {
        std::mem::transmute::<u64x2, u32x4>(v).0
    }
}

macro_rules! _mm_srli_si128 {
    ($v:expr, $ib:expr, $result:ident) => {
        // llvm_asm!(concat!("psrldq xmm1, ", $ib)
        //     : "={xmm1}"($result)
        //     : "{xmm1}"($v)
        //     :
        //     : "intel", "alignstack", "volatile"
        // );
        todo!()
    }
}

#[inline(always)]
pub fn _mm_srli_si128_0x08(v: u64x2) -> u64x2 {
    
    unsafe {
        let r: u128 = std::mem::transmute(v);
        let r = r >> 64;
        std::mem::transmute(r)
    }
    // let r;
    // unsafe {
    //     _mm_srli_si128!(v, 0x08, r)
    // }
    // r
}

macro_rules! mm_slli_si128 {
    ($v:expr, $ib:expr, $result:ident) => {
        // llvm_asm!(concat!("pslldq xmm1, ", $ib)
        //     : "={xmm1}"($result)
        //     : "{xmm1}"($v)
        //     :
        //     : "intel", "alignstack", "volatile"
        // );
        // let $ib = 8 * $ib;
        // asm!(
        //     "lsl v0, v0, {}",
        //     const $ib,
        //     in("v0") $v,
        //     lateout("v0") $result,
        // );
        todo!()
    }
}

#[inline(always)]
pub fn _mm_slli_si128_0x04(v: u64x2) -> u64x2 {
    unsafe {
        let r: u128 = std::mem::transmute(v);
        let r = r << 32;
        std::mem::transmute(r)
    }

    // let r;
    // unsafe {
    //     mm_slli_si128!(v, 0x04, r)
    // }
    // r
}

macro_rules! mm_xor_si128 {
    ($v0:expr, $v1:expr, $result:ident) => {
        // llvm_asm!("pxor xmm1, xmm2"
        //     : "={xmm1}"($result)
        //     : "{xmm1}"($v0), "{xmm2}"($v1)
        //     :
        //     : "intel", "alignstack", "volatile"
        // );
        asm!(
            "eor.16b v2, v1, v0",
            in("v0") $v0,
            in("v1") $v1,
            lateout("v2") $result,
        );
    }
}

#[inline(always)]
pub fn _mm_xor_si128(v0: u64x2, v1: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_xor_si128!(v0, v1, r)
    }
    r
}

macro_rules! mm_mul_su32 {
    ($v0:expr, $v1:expr, $result:ident) => {
        // llvm_asm!("PMULUDQ xmm1, xmm2"
        //     : "={xmm1}"($result)
        //     : "{xmm1}"($v0), "{xmm2}"($v1)
        //     :
        //     : "intel", "alignstack", "volatile"
        // );
        asm!(
            "umull.2d v2, v1, v0",
            in("v0") $v0,
            in("v1") $v1,
            lateout("v2") $result,
        );
    }
}

#[inline(always)]
pub fn _mm_mul_su32(v0: u64x2, v1: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_mul_su32!(v0, v1, r)
    }
    r
}

macro_rules! mm_add_epi64 {
    ($v0:expr, $v1:expr, $result:ident) => {
        // llvm_asm!("PADDQ xmm1, xmm2"
        //     : "={xmm1}"($result)
        //     : "{xmm1}"($v0), "{xmm2}"($v1)
        //     :
        //     : "intel", "alignstack", "volatile"
        // );
        asm!(
            "add.2d v2, v1, v0",
            in("v0") $v0,
            in("v1") $v1,
            lateout("v2") $result,
        );
    }
}

#[inline(always)]
pub fn _mm_add_epi64(v0: u64x2, v1: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_add_epi64!(v0, v1, r)
    }
    r
}

//_mm_sqrt_sd

macro_rules! mm_sqrt_sd {
    ($v0:expr, $v1:expr, $result:ident) => {
        // llvm_asm!("SQRTSD xmm1, xmm2"
        //     : "={xmm1}"($result)
        //     : "{xmm1}"($v0), "{xmm2}"($v1)
        //     :
        //     : "intel", "alignstack", "volatile"
        // );
        asm!(
            "fsqrt.2d v2, v1",
            "ext.16b v2, v0, v2, #8",
            "ext.16b v2, v2, v2, #8",
            in("v0") $v0,
            in("v1") $v1,
            lateout("v2") $result,
        );
    }
}

#[inline(always)]
pub fn _mm_sqrt_sd(v0: u64x2, v1: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_sqrt_sd!(v0, v1, r)
    }
    r
}

//_mm_cvtsi128_si64

macro_rules! mm_cvtsi128_si64 {
    ($v:expr, $result:ident) => {
        asm!(
            "st1.2d {{v0}}[0], [x0]",
            in("v0") $v,
            out("x0") $result,
        )
        // $result = std::mem::transmute::<u64x2, (u64, u64)>($v).0;
    }
}

#[inline(always)]
pub fn _mm_cvtsi128_si64(v: u64x2) -> u64 {
    // let r;
    // unsafe {
    //     mm_cvtsi128_si64!(v, r)
    // }
    v.0
}
