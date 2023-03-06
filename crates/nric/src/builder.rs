use crate::digits::ICDigits;
use crate::nric::NRIC;
use crate::prefix::{ICPrefix, ICPrefixEnum};
use crate::suffix::ICSuffixEnum;

const WEIGHTS: [u8; 7] = [2, 7, 6, 5, 4, 3, 2];

#[derive(Default, Clone)]
pub struct NoICPrefix;
#[derive(Default, Clone)]
pub struct NoICDigits;
#[derive(Default, Clone)]
pub struct NoICSuffix;

#[derive(Default)]
pub struct NRICBuilder<P, D, S> {
    pub prefix: Option<P>,
    pub digits: Option<D>,
    pub suffix: Option<S>,
}

impl NRICBuilder<NoICPrefix, NoICDigits, NoICSuffix> {
    pub fn new() -> Self {
        NRICBuilder::default()
    }
}

impl<D, S> NRICBuilder<NoICPrefix, D, S> {
    pub fn prefix(
        self,
        prefix: impl Into<String>,
    ) -> Result<NRICBuilder<ICPrefix, D, S>, &'static str> {
        let prefix_enum: Result<ICPrefixEnum, _> = prefix.into().parse();
        match prefix_enum {
            Ok(prefix_enum) => Ok(NRICBuilder {
                prefix: Some(ICPrefix(prefix_enum)),
                digits: self.digits,
                suffix: self.suffix,
            }),
            Err(_) => Err("Prefix cannot be parsed."),
        }
    }
}

impl<S> NRICBuilder<ICPrefix, NoICDigits, S> {
    pub fn digits<T: TryInto<ICDigits>>(
        self,
        value: T,
    ) -> Result<NRICBuilder<ICPrefix, ICDigits, S>, &'static str> {
        if let Ok(ic_digits) = value.try_into() {
            Ok(NRICBuilder {
                prefix: self.prefix,
                digits: Some(ic_digits),
                suffix: self.suffix,
            })
        } else {
            Err("Digits cannot be parsed.")
        }
        // match ic_digits {
        //     Ok(ic_digits) => Ok(NRICBuilder {
        //         prefix: self.prefix,
        //         digits: Some(ic_digits),
        //         suffix: self.suffix,
        //     }),
        //     Err(_) => Err("Digits cannot be parsed."),
        // }
    }
}

fn inner_product(arr1: &[u8; 7], arr2: &[u8; 7]) -> u16 {
    arr1.iter()
        .zip(arr2.iter())
        .map(|(&a, &b)| u16::from(a) * u16::from(b))
        .sum()
}

impl NRICBuilder<ICPrefix, ICDigits, NoICSuffix> {
    pub fn suffix(self, suffix: impl Into<String>) -> Result<NRIC, &'static str> {
        let suffix_enum: Result<ICSuffixEnum, _> = suffix.into().parse();
        match suffix_enum {
            Ok(suffix_enum) => {
                let inner_prod = inner_product(&self.digits.as_ref().unwrap().0, &WEIGHTS);
                let prefix_value = self.prefix.as_ref().unwrap().0.value() as u16;
                let validity = (inner_prod + prefix_value) % 11;
                if suffix_enum.value() as u16 == validity {
                    Ok(NRIC {
                        prefix: self.prefix.unwrap().0,
                        digits: self.digits.unwrap(),
                        suffix: suffix_enum,
                    })
                } else {
                    Err("Invalid suffix")
                }
            }
            Err(_) => Err("Suffix cannot be parsed."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        let nric_builder = NRICBuilder::new();
        let nric_builder_with_prefix = nric_builder.prefix("S").unwrap();

        assert_eq!(*nric_builder_with_prefix.prefix.unwrap(), ICPrefixEnum::S);
    }

    #[test]
    fn test_prefix_err() {
        let nric_builder = NRICBuilder::new();
        let nric_builder_with_prefix = nric_builder.prefix("K");

        assert!(nric_builder_with_prefix.is_err());
    }

    #[test]
    fn test_digits_u32() {
        let nric_builder = NRICBuilder::new();
        let nric_builder_with_digits = nric_builder.prefix("S").unwrap().digits(1234567).unwrap();

        assert_eq!(
            nric_builder_with_digits.digits.unwrap(),
            ICDigits([1, 2, 3, 4, 5, 6, 7])
        );
    }

    #[test]
    fn test_digits_u32_err() {
        let nric_builder = NRICBuilder::new();
        let nric_builder_with_digits = nric_builder.prefix("S").unwrap().digits(12345678);

        assert!(nric_builder_with_digits.is_err());
    }

    #[test]
    fn test_digits_str() {
        let nric_builder = NRICBuilder::new();
        let nric_builder_with_digits = nric_builder.prefix("S").unwrap().digits("1234567").unwrap();

        assert_eq!(
            nric_builder_with_digits.digits.unwrap(),
            ICDigits([1, 2, 3, 4, 5, 6, 7])
        );
    }

    #[test]
    fn test_digits_str_err() {
        let nric_builder = NRICBuilder::new();
        let nric_builder_with_digits = nric_builder.prefix("S").unwrap().digits("12345678");

        assert!(nric_builder_with_digits.is_err());
    }

    #[test]
    fn test_inner_product() {
        let array_a = [1, 1, 1, 1, 1, 1, 1];
        let array_b = [1, 1, 1, 1, 1, 1, 1];

        assert_eq!(inner_product(&array_a, &array_b), 7);
    }

    #[test]
    fn test_suffix() {
        let nric = NRICBuilder::new()
            .prefix("T")
            .unwrap()
            .digits("0036447")
            .unwrap()
            .suffix("E")
            .unwrap();
        assert_eq!(nric.suffix, ICSuffixEnum::E);
    }

    #[test]
    fn test_suffix_err() {
        let nric = NRICBuilder::new()
            .prefix("T")
            .unwrap()
            .digits("0036447")
            .unwrap()
            .suffix("F");

        assert!(nric.is_err());
    }
}
