use core::panic;

use crate::{error::ArithmeticError, CoreArith, SoroNum, SoroResult};
use soroban_sdk::Env;

pub trait Power {
    fn pow<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        exponent: u32,
        env: &Env,
    ) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
}

impl Power for SoroNum<i128> {
    fn pow<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        exponent: u32,
        env: &Env,
    ) -> Result<Self, ArithmeticError> {
        let mut result = SoroNum::new(1, SCALE_OUT);
        let mut base = self.clone();
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                let temp = result.mul::<CALC_SCALE, SCALE_OUT>(&base, env, false)?;
                result = match temp {
                    SoroResult::I128(soronum) => soronum,
                    _ => panic!("Invalid type"),
                }
            }
            exp >>= 1;
            if exp > 0 {
                let temp = base.mul::<CALC_SCALE, SCALE_OUT>(&base, env, false)?;
                base = match temp {
                    SoroResult::I128(soronum) => soronum,
                    _ => panic!("Invalid type"),
                }
            }
        }

        Ok(result)
    }
}
