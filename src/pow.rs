use crate::{error::ArithmeticError, SoroNum, CoreArith};


pub trait Power {
    fn pow(self, exponent: u32) -> Result<Self, ArithmeticError> where Self: Sized;
}

impl Power for SoroNum<i128> {
    fn pow(self, exponent: u32) -> Result<Self, ArithmeticError> {
        // Implementation for i128
        let mut result = SoroNum { value: 1 };
        let mut base = self;
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(base)?;
            }
            exp >>= 1;
            if exp > 0 {
                base = base.mul(base)?;
            }
        }

        Ok(result)
    }
}

impl Power for SoroNum<u128> {
    fn pow(self, exponent: u32) -> Result<Self, ArithmeticError> {
        // Implementation for u128
        let mut result = SoroNum { value: 1 };
        let mut base = self;
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(base)?;
            }
            exp >>= 1;
            if exp > 0 {
                base = base.mul(base)?;
            }
        }

        Ok(result)
    }
}
