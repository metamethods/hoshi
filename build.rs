use std::{env, path::Path};

fn main() {
    println!("cargo::rerun-if-changed=locales");
}
