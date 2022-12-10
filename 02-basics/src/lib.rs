// Libraries doesn't have an entrypoint as it is not a program, so we use `lib.rs` when building libraries.
pub fn greeting(name: &str) {
    println!("Hello {}!", name)
}
