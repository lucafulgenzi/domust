#!/usr/bin/env bash

package_version=$(cat Cargo.toml | grep version | cut -d '"' -f2 | head -n1)
cargo clean && cargo build --release
tar -czvf domust-"${package_version}"-x86_64.tar.gz  LICENSE README.md -C target/release domust
cargo clean