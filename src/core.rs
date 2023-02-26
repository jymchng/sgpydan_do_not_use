use crate::builder::{NRICBuilder, NoICDigits, NoICPrefix, NoICSuffix};
use crate::digits::ICDigits;
use crate::prefix::ICPrefixEnum;
use crate::suffix::ICSuffixEnum;
use std::fmt;


#[derive(Debug, Clone)]
pub struct NRIC {
    pub prefix: ICPrefixEnum,
    pub digits: ICDigits,
    pub suffix: ICSuffixEnum,
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
}
