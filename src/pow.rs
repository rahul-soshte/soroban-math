use soroban_sdk::Env;
use crate::{error::ArithmeticError, SoroNum, CoreArith};


pub trait Power {
    fn pow<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        exponent: u32,
        env: &Env
    ) -> Result<Self, ArithmeticError> where Self: Sized;
}

impl Power for SoroNum<i128> {
    fn pow<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        exponent: u32,
        env: &Env
    ) -> Result<Self, ArithmeticError> {

        let mut result = SoroNum::new(1, SCALE_OUT);
        let mut base = self.clone();
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul::<CALC_SCALE, SCALE_OUT>(&base, env)?;
            }
            exp >>= 1;
            if exp > 0 {
                base = base.mul::<CALC_SCALE, SCALE_OUT>(&base, env)?;
            }
        }

        Ok(result)
    }
}

