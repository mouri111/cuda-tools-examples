#![feature(abi_ptx)]

#[macro_use]
extern crate cuda_tools;

use core::cell::UnsafeCell;
use rand::prelude::*;

const KERNEL: &str = include_kernel!();
const N: usize = 1 << 24;

pub fn run() {
    let mut runtime = cuda_tools::runtime::Runtime::new(0, KERNEL).unwrap();
    runtime.record_function_name(vector_add_kernel::vector_add, "vector_add");

    let mut rng = rand::thread_rng();
    let mut xs = vec![];
    for _ in 0..N {
        xs.push(rng.gen());
    }
    let mut ys = vec![];
    for _ in 0..N {
        ys.push(rng.gen());
    }
    let mut zs = vec![];
    for i in 0..N {
        zs.push(UnsafeCell::new(0.0));
    }
    let xs_d = runtime.alloc_slice(&xs).unwrap();
    let ys_d = runtime.alloc_slice(&ys).unwrap();
    let zs_d = runtime.alloc_slice(&zs).unwrap();
    let args = vector_add_kernel::Arguments {
        xs: xs_d,
        ys: ys_d,
        zs: zs_d,
    };

    runtime
        .launch(
            vector_add_kernel::vector_add,
            &args,
            N / 256,
            1,
            1,
            256,
            1,
            1,
        )
        .unwrap();

    let zs = args.zs.to_host().unwrap();
    let zs: Vec<f32> = zs.into_iter().map(|x| x.into_inner()).collect();
    for i in 0..N {
        assert!((zs[i] - (xs[i] + ys[i])).abs() < 1e-5);
    }
    println!("ok");
}
