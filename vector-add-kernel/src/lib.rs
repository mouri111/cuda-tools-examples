#![no_std]
#![feature(abi_ptx)]
#![feature(stdsimd)]

#[macro_use]
extern crate cuda_tools;

use cuda_tools::cuda_slice::CUDASlice;
use core::cell::UnsafeCell;

pub struct Arguments<'a> {
    pub xs: CUDASlice<'a, f32>,
    pub ys: CUDASlice<'a, f32>,
    pub zs: CUDASlice<'a, UnsafeCell<f32>>,
}

#[no_mangle]
#[cfg(not(target_arch = "nvptx64"))]
pub extern "ptx-kernel" fn vector_add(args: &Arguments) {}

#[no_mangle]
#[cfg(target_arch = "nvptx64")]
pub extern "ptx-kernel" fn vector_add(args: &Arguments) {
    let i = unsafe {
        use core::arch::nvptx::*;
        _block_dim_x() * _block_idx_x() + _thread_idx_x()
    } as usize;
    unsafe {
        if i < args.zs.len() && i < args.xs.len() && i < args.ys.len() {
            *args.zs[i].get() = args.xs[i] + args.ys[i];
        }
    }
}
