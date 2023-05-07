use std::ops::Deref;
use strum_macros::{self, Display, EnumString};

#[derive(Debug, Eq, PartialEq, Display, EnumString, Clone)]
#[strum(serialize_all = "UPPERCASE")]
#[strum(ascii_case_insensitive)]
pub enum ICSuffixEnum {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    Z,
    J,
    K,
    L,
    M,
    N,
    P,
    Q,
    R,
    T,
    U,
    W,
    X,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ICSuffix(pub(crate) ICSuffixEnum);

impl ICSuffixEnum {
    // pub fn new(suffix_enum: ICSuffixEnum) -> Self {
    //   Self(suffix_enum)
    // }

    pub fn value(&self) -> u8 {
        match self {
            ICSuffixEnum::X | ICSuffixEnum::J => 0,
            ICSuffixEnum::W | ICSuffixEnum::Z => 1,
            ICSuffixEnum::U | ICSuffixEnum::I => 2,
            ICSuffixEnum::T | ICSuffixEnum::H => 3,
            ICSuffixEnum::R | ICSuffixEnum::G => 4,
            ICSuffixEnum::Q | ICSuffixEnum::F => 5,
            ICSuffixEnum::P | ICSuffixEnum::E => 6,
            ICSuffixEnum::N | ICSuffixEnum::D => 7,
            ICSuffixEnum::M | ICSuffixEnum::C => 8,
            ICSuffixEnum::L | ICSuffixEnum::B => 9,
            ICSuffixEnum::K | ICSuffixEnum::A => 10,
        }
    }
}

impl Deref for ICSuffix {
    type Target = ICSuffixEnum;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
