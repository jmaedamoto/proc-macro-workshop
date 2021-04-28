use bitfield::*;

const F: isize = 1;

#[derive(BitfieldSpecifier)]
pub enum DeliveryMode {
    Fixed = F,
    Lowest,
    SMI,
    RemoteRead,
    NMI,
    Init,
    Startup,
    External,
}

fn main() {}
