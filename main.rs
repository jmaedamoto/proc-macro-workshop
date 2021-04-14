// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use bitfield::*;

// #[bitfield]
// pub struct MyFourBytes {
//     a: B1,
//     b: B3,
//     c: B4,
//     d: B24,
// }

#[repr(C)]
pub struct MyFourBytes {
    data: [u8; (0
        + <B1 as Specifier>::BITS
        + <B3 as Specifier>::BITS
        + <B14 as Specifier>::BITS
        + <B6 as Specifier>::BITS)
        / 8],
}

impl MyFourBytes {
    fn new() -> MyFourBytes {
        MyFourBytes {
            data: [0u8; (0
                + <B1 as Specifier>::BITS
                + <B3 as Specifier>::BITS
                + <B13 as Specifier>::BITS
                + <B7 as Specifier>::BITS)
                / 8],
        }
    }

    fn set_c(&mut self, value: u64){
        let start_byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) / 8;
        let start_bit = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) % 8;
        let end_byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS + <B14 as Specifier>::BITS) / 8;
        let end_bit = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS + <B14 as Specifier>::BITS) % 8;
        let size = <B14 as Specifier>::BITS;

        //clear existing data.
        self.data[start_byte] = self.data[start_byte].checked_shl((8 - start_bit) as u32).unwrap();
        self.data[start_byte] = self.data[start_byte].checked_shr((8 - start_bit) as u32).unwrap();
        self.data[end_byte] = self.data[end_byte].checked_shr(end_bit as u32).unwrap();
        self.data[end_byte] = self.data[end_byte].checked_shl(end_bit as u32).unwrap();

        if end_byte >  start_byte {
            for i in (start_byte + 1)..=end_byte{
                self.data[i] = 0;
            }
        }

        let value_start_byte = value.checked_shl((size - 8 + start_bit) as u32).unwrap();
        let mut value_start_byte = value_start_byte.checked_shr((size - 8) as u32).unwrap() as u8;
        if start_byte == end_byte {
            value_start_byte = value_start_byte.checked_shr((8 - end_bit) as u32).unwrap();
            self.data[start_byte] = self.data[start_byte] | value_start_byte;
        }else{
            let value_end_byte = value.checked_shr((size - end_bit) as u32).unwrap() as u8;
            dbg!(format!("{:b}",value_end_byte));
            self.data[start_byte] = self.data[start_byte] | value_start_byte;
            self.data[end_byte] = self.data[end_byte] | value_end_byte;
            for i in (start_byte + 1)..end_byte{
                let value_i_byte = value.checked_shr((start_bit + 8 * (i - start_byte -1)) as u32).unwrap() as u8;
                self.data[i] = self.data[i] | value_i_byte;
            }
        }       
    }

    fn get_c(&mut self) -> u64{
        let start_byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) / 8;
        let start_bit = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) % 8;
        let end_byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS + <B14 as Specifier>::BITS) / 8;
        let end_bit = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS + <B14 as Specifier>::BITS) % 8;

        let mut value = self.data[start_byte] as <B14 as Specifier>::UNIT;
        if start_byte == end_byte{
            value = value.checked_shl((8 - end_bit) as u32).unwrap();
            value = value.checked_shr((8 - end_bit + start_bit) as u32).unwrap();
        }else{
            for i in (start_byte + 1)..end_byte {
                value += (self.data[i] as <B14 as Specifier>::UNIT).checked_shl((start_bit + 8 * (i - start_byte - 1)) as u32).unwrap(); 
            }
            let mut  byte = self.data[end_byte] as  <B14 as Specifier>::UNIT;
            byte = byte.checked_shl((8 - end_bit) as u32).unwrap();
            byte = byte.checked_shr((8 - end_bit + start_bit) as u32).unwrap();
        }

        (self.data[start_byte].checked_shr(start_bit as u32).unwrap()) as u64
    }
}

fn main() {
    let mut bitfield = MyFourBytes::new();
    //assert_eq!(0, bitfield.get_a());
    //assert_eq!(0, bitfield.get_b());
    //assert_eq!(0, bitfield.get_c());
    //assert_eq!(0, bitfield.get_d());

    bitfield.set_c(8191);
    //assert_eq!(0, bitfield.get_a());
    //assert_eq!(0, bitfield.get_b());
    assert_eq!(8191, bitfield.get_c());
    //assert_eq!(0, bitfield.get_d());
}