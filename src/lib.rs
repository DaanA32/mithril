#![crate_name = "mithril"]
#![crate_type = "lib"]

#![feature(llvm_asm)]
#![feature(asm)]
#![feature(repr_simd)]
#![feature(stdsimd)]
#![feature(box_syntax)]
#![feature(integer_atomics)]
#![feature(aarch64_target_feature)]
#![allow(unreachable_patterns)]
#![allow(clippy::unreachable)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate strum;

pub mod byte_string;
pub mod cryptonight;
pub mod stratum;
pub mod worker;
pub mod u64x2;
pub mod metric;
pub mod bandit_tools;
pub mod mithril_config;
pub mod timer;
pub mod randomx;

// macro_rules! info {
//     () => {
        
//     };
// }