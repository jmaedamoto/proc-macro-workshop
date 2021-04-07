// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run


use seq::seq;

seq!(N in 0..1 {
    fn main() {
        let _ = Missing#N;
    }
});
