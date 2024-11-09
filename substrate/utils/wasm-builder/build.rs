// SPDX-License-Identifier: Apache-2.0

use std::process::Command;

fn main() {
    Command::new("rustup")
        .args(["override", "set", "nightly-2024-11-01"]).output().expect("rustup");

    println!("cargo::rerun-if-changed=build.rs");
}
