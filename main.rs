// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use derive_debug::CustomDebug;
use std::fmt::Debug;

#[derive(CustomDebug)]
pub struct One<T> {
    value: T,
    two: Option<Box<Two<T>>>,
}

#[derive(CustomDebug)]
struct Two<T> {
    one: Box<One<T>>,
}

fn assert_debug<F: Debug>() {}

fn main() {
    assert_debug::<One<u8>>();
    assert_debug::<Two<u8>>();
}