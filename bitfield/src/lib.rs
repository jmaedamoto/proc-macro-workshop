// Crates that have the "proc-macro" crate type are only allowed to export
// procedural macros. So we cannot have one crate that defines procedural macros
// alongside other types of public APIs like traits and structs.
//
// For this project we are going to need a #[bitfield] macro but also a trait
// and some structs. We solve this by defining the trait and structs in this
// crate, defining the attribute macro in a separate bitfield-impl crate, and
// then re-exporting the macro from this crate so that users only have one crate
// that they need to import.
//
// From the perspective of a user of this crate, they get all the necessary APIs
// (macro, trait, struct) through the one bitfield crate.
// 
// Todo:
// toとfromを何らかの形で実装する

pub use bitfield_impl::*;
pub mod check;

pub trait Specifier {
  type UNIT;
  type InOut;
  const BITS : usize;

  fn from_bytes(bytes: Self::UNIT) -> Self::InOut;
  fn to_bytes(input: Self::InOut) -> Self::UNIT;
}

bitfield_impl::define_default_type!{}

impl Specifier for bool{
  type UNIT = u8;
  type InOut = bool;
  const BITS : usize = 1;

  fn from_bytes(bytes: u8) -> bool {
    match bytes{
      0 => false,
      1 => true,
      _ => unreachable!()
    }
  }

  fn to_bytes(input: bool) -> u8 {
    input as u8
  }
}