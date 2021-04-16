use bitfield::*;
use std::mem::size_of_val;

type A = B1;
type B = B3;
type C = B4;
type D = B24;

#[bitfield]
pub struct MyFourBytes {
    a: A,
    b: B,
    c: C,
    d: D,
}

fn main() {
    let mut x = MyFourBytes::new();

    // // I am testing the signatures in this roundabout way to avoid making it
    // // possible to pass this test with a generic signature that is inconvenient
    // // for callers, such as `fn get_a<T: From<u64>>(&self) -> T`.

    let a = 1;
    assert_eq!(size_of_val(&a), 1);
    x.set_a(a); // expect fn(&mut MyFourBytes, u8)
    let b = 1;
    x.set_b(b);
    let c = 1;
    x.set_c(c);
    let d = 1;
    x.set_d(d); // expect fn(&mut MyFourBytes, u32)

    assert_eq!(size_of_val(&a), 1);
    // assert_eq!(size_of_val(&b), 1);
    // assert_eq!(size_of_val(&c), 1);
    // assert_eq!(size_of_val(&d), 4);

    assert_eq!(size_of_val(&x.get_a()), 1); // expect fn(&MyFourBytes) -> u8
    // assert_eq!(size_of_val(&x.get_b()), 1);
    // assert_eq!(size_of_val(&x.get_c()), 1);
    // assert_eq!(size_of_val(&x.get_d()), 4); // expect fn(&MyFourBytes) -> u32
}
