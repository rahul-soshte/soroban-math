use core::panic;

use crate::{error::ArithmeticError, CoreArith, SoroNum, SoroResult};
use soroban_sdk::Env;
use soroban_sdk::I256;

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


impl Power for SoroNum<I256> {
    fn pow<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        exponent: u32,
        env: &Env,
    ) -> Result<Self, ArithmeticError> {
        let mut result = SoroNum::new(I256::from_i32(env, 1), SCALE_OUT);
        let mut base = self.clone();
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                let temp = result.mul::<CALC_SCALE, SCALE_OUT>(&base, env, true)?;
                result = match temp {
                    SoroResult::I256(soronum) => soronum,
                    _ => panic!("Invalid type"),
                }
            }
            exp >>= 1;
            if exp > 0 {
                let temp = base.mul::<CALC_SCALE, SCALE_OUT>(&base, env, true)?;
                base = match temp {
                    SoroResult::I256(soronum) => soronum,
                    _ => panic!("Invalid type"),
                }
            }
        }

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    
    mod soronum_i128_pow_tests {

        #[test]
        fn test_pow_positive_exponent() {
            todo!();
        }
    }

    mod soronum_i256_pow_tests {

        #[test]
        fn test_pow_positive_exponent() {
            todo!();
        }
    }
}