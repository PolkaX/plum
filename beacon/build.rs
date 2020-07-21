// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

fn main() {
    prost_build::compile_protos(
        &["proto/drand/api.proto", "proto/drand/common.proto"],
        &["proto"],
    )
    .expect("Compile proto shouldn't be fail");
    println!("cargo:rerun-if-changed=proto");
}
