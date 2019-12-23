#[macro_use]
extern crate cuda_tools;

fn main() {
    build_kernel!("../vector-add-kernel", "vector-add-kernel");
}
