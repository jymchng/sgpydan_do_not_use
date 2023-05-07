use std::ops::Deref;
use strum_macros::{self, Display, EnumString};

#[derive(Debug, Eq, PartialEq, Display, EnumString, Clone)]
#[strum(serialize_all = "UPPERCASE")]
#[strum(ascii_case_insensitive)]
pub enum ICPrefixEnum {
    T,
    S,
    G,
    F,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ICPrefix(pub(crate) ICPrefixEnum);

impl ICPrefixEnum {
    pub fn value(&self) -> u8 {
        match self {
            ICPrefixEnum::F | ICPrefixEnum::S => 0,
            _ => 4,
        }
    }
}

impl Deref for ICPrefix {
    type Target = ICPrefixEnum;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_value_if_st() {
        assert_eq!(ICPrefixEnum::S.value(), 0);
        assert_eq!(ICPrefixEnum::G.value(), 4)
    }
}
