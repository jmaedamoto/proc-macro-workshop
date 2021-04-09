// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run
#[sorted::check]
fn f(bytes: &[u8]) -> Option<u8> {
    #[sorted]
    match bytes {
        [] => Some(0),
        [a] => Some(*a),
        [a, b] => Some(a + b),
        _other => None,
    }
}

fn main() {}
