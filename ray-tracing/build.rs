#[macro_use]
extern crate cuda_tools;

fn main() {
    build_kernel!("../ray-tracing-kernel", "ray-tracing-kernel");
}
