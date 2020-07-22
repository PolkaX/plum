// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

fn main() {
    protobuf_build::Builder::new()
        .search_dir_for_protos("proto/drand")
        .package_name("drand")
        .generate();
}
