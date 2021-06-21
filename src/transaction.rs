use crate::helper_types::{DECIMAL_PRECISION, FLOAT_BASE};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Amount(pub u64);

impl FromStr for Amount {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decimal_index = s.find('.').ok_or_else(|| "expected decimal")?;
        let decimal_slice = &s[decimal_index + 1..];
        if decimal_slice.len() > DECIMAL_PRECISION {
            return Err(format!("at most {} decimal digits allowed", DECIMAL_PRECISION).into());
        }
        let missing_zeros = DECIMAL_PRECISION - decimal_slice.len();
        let decimal = decimal_slice.parse::<u64>()?;
        let decimal = decimal
            .checked_mul(10u64.pow(missing_zeros as u32))
            .unwrap();
        let whole_number = s[..decimal_index]
            .parse::<u64>()?
            .checked_mul(FLOAT_BASE as u64)
            .ok_or_else(|| "Overflow")?;
        Ok(Self(
            whole_number
                .checked_add(decimal)
                .ok_or_else(|| "Overflow")?,
        ))
    }
}
