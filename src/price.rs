use core::fmt;
use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use crate::errors::TicketError;

const SCALE: u128 = 1_000;
const SCALE_DECIMAL: u32 = 3;

fn str_parser(value: &str) -> Result<u128, TicketError> {
    let (whole, frac) = value.split_once(".").unwrap_or((value, "0"));

    let frac_length = frac.len() as u32;

    if frac_length > SCALE_DECIMAL {
        return Err(TicketError::PriceError("Too many decimal places".into()));
    }

    // parse numbers from str

    let whole: u128 = whole
        .parse()
        .map_err(|_| TicketError::PriceError("Invalid Number".into()))?;

    let frac: u128 = frac
        .parse()
        .map_err(|_| TicketError::PriceError("Invalid Decimal Places".into()))?;

    // scale and normalize

    let whole_scaled: u128 = whole
        .checked_mul(SCALE)
        .ok_or(TicketError::PriceError("Number too large".into()))?;

    let frac_scaled: u128 = frac
        .checked_mul((10u128).pow(SCALE_DECIMAL - frac_length))
        .ok_or(TicketError::PriceError("".into()))?;

    // add whole_scaled and frac_scaled num

    let value = whole_scaled
        .checked_add(frac_scaled)
        .ok_or(TicketError::PriceError("Number Too Large".into()))?;

    Ok(value)
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Price {
    value: u128,
}

impl FromStr for Price {
    type Err = TicketError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value: u128 = str_parser(value)?;
        Ok(Self { value })
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = self.value / SCALE;
        let frac = self.value % SCALE;
        write!(
            f,
            "{}.{:0width$}",
            whole,
            frac,
            width = SCALE_DECIMAL as usize
        )
    }
}

impl Add for Price {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.checked_add(rhs.value).unwrap(),
        }
    }
}
impl Add<u64> for Price {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        let aval: u128 = (rhs as u128)
            .checked_mul(SCALE)
            .expect("Overflow Error: Number Too Big");
        Self {
            value: self
                .value
                .checked_add(aval)
                .expect("Overflow Error: Number Too Big"),
        }
    }
}

impl Sub for Price {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.checked_sub(rhs.value).unwrap(),
        }
    }
}

impl Sub<u64> for Price {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        let aval: u128 = (rhs as u128)
            .checked_mul(SCALE)
            .expect("Overflow Error: Number Too Big");
        Self {
            value: self
                .value
                .checked_sub(aval)
                .expect("Overflow Error: Number Too Big"),
        }
    }
}

impl Div for Price {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.checked_div(rhs.value).unwrap(),
        }
    }
}

impl Div<u64> for Price {
    type Output = Self;
    fn div(self, rhs: u64) -> Self::Output {
        let aval: u128 = rhs as u128;
        Self {
            value: self
                .value
                .checked_div(aval)
                .expect("Overflow Error: Number Too Big"),
        }
    }
}

impl Mul for Price {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.checked_mul(rhs.value).unwrap(),
        }
    }
}

impl Mul<u64> for Price {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        let aval: u128 = rhs as u128;
        Self {
            value: self
                .value
                .checked_mul(aval)
                .expect("Overflow Error: Number Too Big"),
        }
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<&str> for Price {
    fn eq(&self, other: &&str) -> bool {
        let value = match str_parser(other) {
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };
        self.value == value
    }
}

impl Eq for Price {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn price() -> Price {
        Price::from_str("400.00").unwrap()
    }

    #[rstest]
    fn test_new_price(price: Price) {
        assert_eq!(price, "400.00");
    }

    #[rstest]
    fn test_price_add(price: Price) {
        let pp: u64 = 40;
        assert_eq!(price + pp, "440")
    }

    #[rstest]
    fn test_price_subtract(price: Price) {
        let pp: u64 = 40;
        assert_eq!(price - pp, "360")
    }

    #[rstest]
    fn test_price_divide(price: Price) {
        let pp: u64 = 40;
        assert_eq!(price / pp, "10")
    }

    #[rstest]
    fn test_price_multiply(price: Price) {
        let pp: u64 = 40;
        assert_eq!(price * pp, "16000")
    }
}
