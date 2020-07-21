// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    protobuf_build::Builder::new()
        .includes(&["proto"])
        .files(&["proto/drand/api.proto", "proto/drand/common.proto"])
        .out_dir(out_dir)
        .generate();
}
