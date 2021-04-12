// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use bitfield::*;

#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}

// #[repr(C)]
// pub struct MyFourBytes {
//     data: [u8; (0
//         + <B1 as Specifier>::BITS
//         + <B3 as Specifier>::BITS
//         + <B4 as Specifier>::BITS
//         + <B24 as Specifier>::BITS)
//         / 8],
// }
// impl MyFourBytes {
//     fn new() -> MyFourBytes {
//         MyFourBytes {
//             data: [0u8; (0
//                 + <B1 as Specifier>::BITS
//                 + <B3 as Specifier>::BITS
//                 + <B4 as Specifier>::BITS
//                 + <B24 as Specifier>::BITS)
//                 / 8],
//         }
//     }

//     fn set_c(&mut self,c: u64){
//         let byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) / 8;
//         let bit = <B1 as Specifier>::BITS + <B3 as Specifier>::BITS;

//         let mut mask:[u8;4] = [255,255,255,255];
//         mask[byte] = mask[byte].checked_shl(bit as u32).unwrap() as u8;

//         let mut value:[u8;4] = [0,0,0,0];
//         value[byte] = c.checked_shl(bit as u32).unwrap() as u8;

//         for i in 0..self.data.len(){
//             self.data[i] = self.data[i] & mask[i] | value[i];
//         }
//     }

//     fn get_c(&mut self) -> u64{
//         let byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) / 8;
//         let bit = <B1 as Specifier>::BITS + <B3 as Specifier>::BITS;
//         (self.data[byte].checked_shr(bit as u32).unwrap()) as u64
//     }
// }

fn main() {
    let mut bitfield = MyFourBytes::new();
    //assert_eq!(0, bitfield.get_a());
    //assert_eq!(0, bitfield.get_b());
    //assert_eq!(0, bitfield.get_c());
    //assert_eq!(0, bitfield.get_d());

    //bitfield.set_c(14);
    //assert_eq!(0, bitfield.get_a());
    //assert_eq!(0, bitfield.get_b());
    //assert_eq!(14, bitfield.get_c());
    //assert_eq!(0, bitfield.get_d());
}
