cuda-tools-examples/vector-add
===

## Setup

1. Install nightly compiler.
For example, 
```
$ rustup default nightly
```

2. Install nvptx64-nvidia-cuda target.
```
$ rustup target add nvptx64-nvidia-cuda
```

3. Set `CUDA_PATH` environment  variable (if needed).
    e.g. "/opt/cuda"

## Run
```
$ cargo run --release
```
