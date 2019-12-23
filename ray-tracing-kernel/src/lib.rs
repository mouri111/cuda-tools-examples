#![no_std]
#![feature(abi_ptx)]
#![feature(stdsimd)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate cuda_tools;

pub mod vec3;
pub mod xorshift;
pub mod camera;
pub mod hit_record;
pub mod object;
pub mod ray_trace_args;
pub mod ray;
pub mod kernel;
