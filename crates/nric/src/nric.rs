use crate::builder::NRICBuilder;
use crate::digits::ICDigits;
use crate::prefix::ICPrefixEnum;
use crate::suffix::ICSuffixEnum;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NRIC {
    pub(crate) prefix: ICPrefixEnum,
    pub(crate) digits: ICDigits,
    pub(crate) suffix: ICSuffixEnum,
}

impl fmt::Display for NRIC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.prefix, self.digits, self.suffix)
    }
}

impl NRIC {
    pub fn new(string: impl Into<String>) -> Result<NRIC, &'static str> {
        let string = string.into();
        let first_letter = &string[0..1]; // slice from index 0 to 1 (exclusive)
        let last_letter = &string[string.len() - 1..]; // slice from the last index to the end
        let digits: String = string.chars().filter(|c| c.is_digit(10)).collect(); // collect digits as string
        Ok(NRICBuilder::new()
            .prefix(first_letter)?
            .digits(digits)?
            .suffix(last_letter)?)
    }

    pub fn prefix(&self) -> String {
        self.prefix.to_string()
    }

    pub fn suffix(&self) -> String {
        self.suffix.to_string()
    }

    pub fn digits(&self) -> String {
        self.digits.to_string()
    }
}
