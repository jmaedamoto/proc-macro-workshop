pub trait SizeType{
  type CheckType;
}

pub struct TotalSize<T>(::std::marker::PhantomData<T>);

pub trait TotalSizeIsMultipleOfEightBits{}

macro_rules! impl_total_size_for {
  ($(($n:expr,$name:ident)),*) => {
      $(
          pub enum $name {}
          impl SizeType for TotalSize<[();$n]>{
              type CheckType = $name;
          }
      )* 
  };
}

impl_total_size_for!(
  (0, ZeroMod8),
  (1, OneMod8),
  (2, TwoMod8),
  (3, ThreeMod8),
  (4, FourMod8),
  (5, FiveMod8),
  (6, SixMod8),
  (7, SevenMod8)
);

impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {}
pub trait CheckTotalSizeMultipleOf8
where <Self::Size as SizeType>::CheckType: TotalSizeIsMultipleOfEightBits{
  type Size: SizeType;
}


pub trait DiscriminantInRange{}

pub enum True{}
pub enum False{}

pub trait DispatchTrueFalse{
  type Out;
}

impl DiscriminantInRange for True{}

impl DispatchTrueFalse for [(); 0]{
  type Out = False;
}

impl DispatchTrueFalse for [(); 1]{
  type Out = True;
}

pub trait CheckDiscriminantInRange<A>
where <Self::CheckType as DispatchTrueFalse>::Out: DiscriminantInRange{
  type CheckType: DispatchTrueFalse;
}