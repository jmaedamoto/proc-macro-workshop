use bitfield::*;

#[bitfield]
pub struct RedirectionTableEntry {
    acknowledged: bool,
    trigger_mode: TriggerMode,
    delivery_mode: DeliveryMode,
    reserved: B3,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum DeliveryMode {
    Fixed = 0b000,
    Lowest = 0b001,
    SMI = 0b010,
    RemoteRead = 0b011,
    NMI = 0b100,
    Init = 0b101,
    Startup = 0b110,
    External = 0b111,
}

fn main() {
    assert_eq!(std::mem::size_of::<RedirectionTableEntry>(), 1);

    // Initialized to all 0 bits.
    let mut entry = RedirectionTableEntry::new();
    //assert_eq!(entry.get_acknowledged(), false);
    //assert_eq!(entry.get_trigger_mode(), TriggerMode::Edge);
    //assert_eq!(entry.get_delivery_mode(), DeliveryMode::Fixed);

    entry.set_acknowledged(true);
    entry.set_delivery_mode(DeliveryMode::SMI);
    assert_eq!(entry.get_acknowledged(), true);
    assert_eq!(entry.get_trigger_mode(), TriggerMode::Edge);
    assert_eq!(entry.get_delivery_mode(), DeliveryMode::SMI);
}