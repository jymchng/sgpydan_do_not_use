use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ICDigits(pub [u8; 7]);

impl ICDigits {
    pub fn new(values: [u8; 7]) -> Result<Self, &'static str> {
        let all_val_smaller_than_nine = { values.iter().all(|&val| val <= 9) };
        if all_val_smaller_than_nine {
            return Ok(Self(values));
        }
        Err("The number of digits for NRIC is not equal to 7")
    }
}

impl Deref for ICDigits {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<u32> for ICDigits {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let mut digits = [0; 7];
        let mut remaining = value;

        for i in (0..7).rev() {
            let digit = remaining % 10;
            remaining /= 10;

            digits[i] = digit as u8;
        }

        if remaining > 0 {
            Err("Number is too large to fit in an array of 7 digits")
        } else {
            Ok(ICDigits(digits))
        }
    }
}

impl TryFrom<&str> for ICDigits {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let string_value: String = value.into();
        if string_value.len() > 7 {
            return Err("String is too long to fit in an array of 7 digits");
        }
        let mut digits = [0; 7];
        for (i, c) in string_value.chars().enumerate() {
            match c.to_digit(10) {
                Some(digit) if i < 7 => digits[i] = digit as u8,
                _ => return Err("String contains non-digit characters"),
            }
        }
        Ok(ICDigits(digits))
    }
}

impl TryFrom<String> for ICDigits {
    type Error = &'static str;

    fn try_from(string_value: String) -> Result<Self, Self::Error> {
        if string_value.len() > 7 {
            return Err("String is too long to fit in an array of 7 digits");
        }
        let mut digits = [0; 7];
        for (i, c) in string_value.chars().enumerate() {
            match c.to_digit(10) {
                Some(digit) if i < 7 => digits[i] = digit as u8,
                _ => return Err("String contains non-digit characters"),
            }
        }
        Ok(ICDigits(digits))
    }
}

impl TryFrom<Vec<u8>> for ICDigits {
    type Error = &'static str;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > 7 {
            return Err("Vector is too long to fit in an array of 7 digits");
        }
        let mut digits = [0; 7];
        for (i, &digit) in value.iter().enumerate() {
            if i >= 7 {
                break;
            }
            digits[i] = digit;
        }
        Ok(ICDigits(digits))
    }
}

impl fmt::Display for ICDigits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.0.iter().map(|&d| d.to_string()).collect();
        write!(f, "{}", s)
    }
}

impl ICDigits {
    pub fn try_parse<T: TryInto<ICDigits>>(value: T) -> Result<Self, T::Error> {
        value.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icdigits_new() {
        let result = ICDigits::new([1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result.is_ok(), true);
        let result = ICDigits::new([10, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_icdigits_try_from_u32() {
        let result = ICDigits::try_from(1234567);
        assert_eq!(result.is_ok(), true);
        let result = ICDigits::try_from(123456789);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_icdigits_try_from_str() {
        let result = ICDigits::try_from("1234567");
        assert_eq!(result.is_ok(), true);
        let result = ICDigits::try_from("123456789");
        assert_eq!(result.is_err(), true);
        let result = ICDigits::try_from("1234567a");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_icdigits_try_from_vec() {
        let result = ICDigits::try_from(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result.is_ok(), true);
        let result = ICDigits::try_from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_icdigits_deref() {
        let icdigits = ICDigits::new([1, 2, 3, 4, 5, 6, 7]).unwrap();
        let slice: &[u8] = &icdigits;
        assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_try_parse_u32() {
        let value = 1234567;
        let ic_digits = ICDigits::try_parse(value).unwrap();
        assert_eq!(ic_digits, ICDigits::new([1, 2, 3, 4, 5, 6, 7]).unwrap());
    }

    #[test]
    fn test_try_parse_str() {
        let value = "1234567";
        let ic_digits = ICDigits::try_parse(value).unwrap();
        assert_eq!(ic_digits, ICDigits::new([1, 2, 3, 4, 5, 6, 7]).unwrap());
    }

    #[test]
    fn test_try_parse_vec() {
        let value = vec![1, 2, 3, 4, 5, 6, 7];
        let ic_digits = ICDigits::try_parse(value).unwrap();
        assert_eq!(ic_digits, ICDigits::new([1, 2, 3, 4, 5, 6, 7]).unwrap());
    }

    #[test]
    fn test_try_parse_invalid_str() {
        let value = "12345678";
        let ic_digits_result = ICDigits::try_parse(value);
        assert!(ic_digits_result.is_err());
        assert_eq!(
            ic_digits_result.unwrap_err(),
            "String is too long to fit in an array of 7 digits"
        );
    }

    #[test]
    fn test_try_parse_invalid_vec() {
        let value = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let ic_digits_result = ICDigits::try_parse(value);
        assert!(ic_digits_result.is_err());
        assert_eq!(
            ic_digits_result.unwrap_err(),
            "Vector is too long to fit in an array of 7 digits"
        );
    }
}
