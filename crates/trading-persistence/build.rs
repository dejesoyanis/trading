//! Rebuilds the persistence crate when infrastructure migrations change.

fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
