use mmrbi::*;

fn main() {
    warning!("this is a test warning");
    error!(code: "EX0001", "this is a test error");
    info!(at: "examples/macros.rs", "this is a test message");
}
