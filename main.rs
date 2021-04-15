// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use bitfield::*;

//#[bitfield]
// pub struct MyFourBytes {
//     a: B1,
//     b: B3,
//     c: B4,
//     d: B24,
// }
const SIZE:usize = (0+ <B1 as Specifier>::BITS+ <B3 as Specifier>::BITS+ <B5 as Specifier>::BITS+ <B23 as Specifier>::BITS) / 8;
#[repr(C)]
pub struct MyFourBytes {
    data: [u8; (0
        + <B1 as Specifier>::BITS
        + <B3 as Specifier>::BITS
        + <B5 as Specifier>::BITS
        + <B23 as Specifier>::BITS)
        / 8],
}
impl MyFourBytes {
    fn new() -> MyFourBytes {
        MyFourBytes {
            data: [0u8; (0
                + <B1 as Specifier>::BITS
                + <B3 as Specifier>::BITS
                + <B5 as Specifier>::BITS
                + <B23 as Specifier>::BITS)
                / 8],
        }
    }

    fn set_c(&mut self,c: u64){
        let start_byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) / 8;
        let start_bit = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) % 8;
        let end_byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS  + <B5 as Specifier>::BITS) / 8;
        let end_bit = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS  + <B5 as Specifier>::BITS) % 8;
        let size = <B5 as Specifier>::BITS;

        dbg!(start_byte);
        dbg!(start_bit);
        dbg!(end_byte);
        dbg!(end_bit);
        
        let mut mask:[u8; SIZE] = [255; SIZE];
        let mut value:[u8; SIZE] = [0; SIZE];
        if start_byte == end_byte{
            mask[start_byte] = 255u8.checked_shl(start_bit as u32).unwrap() & 255u8.checked_shr((8 - end_bit) as u32).unwrap();
            value[start_byte] =  c.checked_shl(start_bit as u32).unwrap() as u8
        }else{
            mask[start_byte] = 255u8.checked_shl(start_bit as u32).unwrap();
            mask[end_byte] = 255u8.checked_shr((8 - end_bit) as u32).unwrap();
            value[start_byte] =  c.checked_shl(start_bit as u32).unwrap() as u8;
            value[end_byte] = c.checked_shr((size - end_bit) as u32).unwrap() as u8;
            if start_byte + 1 < end_byte{
                for i in (start_byte + 1 ) ..end_byte{
                    mask[i] = 255;
                }
            }
        }

        for i in 0..self.data.len(){
            self.data[i] = self.data[i] & mask[i] | value[i];
        }
    }

    fn get_c(&mut self) -> u64{
        let start_byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) / 8;
        let start_bit = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) % 8;
        let end_byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS  + <B5 as Specifier>::BITS) / 8;
        let end_bit = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS  + <B5 as Specifier>::BITS) % 8;
        let size = <B5 as Specifier>::BITS;

        u32::from_be_bytes(self.data);
    }
}

fn main() {
    let mut bitfield = MyFourBytes::new();
//   assert_eq!(0, bitfield.get_a());
//    assert_eq!(0, bitfield.get_b());
//   assert_eq!(0, bitfield.get_c());
//    assert_eq!(0, bitfield.get_d());

    bitfield.set_c(28);
    // assert_eq!(0, bitfield.get_a());
    // assert_eq!(0, bitfield.get_b());
    assert_eq!(28, bitfield.get_c());
    // assert_eq!(0, bitfield.get_d());
}
