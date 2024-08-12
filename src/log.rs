use soroban_sdk::{Bytes, Env, I256};

use crate::{error::ArithmeticError, SoroNum};
pub trait Logarithm {
    fn log2<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env,
    ) -> Result<Self, ArithmeticError>
    where
        Self: Sized;

    fn log10<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env,
    ) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
}

pub trait I256Extensions {
    fn leading_zeros(&self) -> u32;
}

impl I256Extensions for I256 {
    fn leading_zeros(&self) -> u32 {
        let bytes: Bytes = self.to_be_bytes();
        let mut leading_zeros = 0;

        for byte_index in 0..bytes.len() {
            let byte_value = bytes.get(byte_index).unwrap_or(0);
            if byte_value == 0 {
                leading_zeros += 8; // Each byte has 8 bits.
            } else {
                leading_zeros += byte_value.leading_zeros();
                break;
            }
        }

        leading_zeros
    }
}

impl Logarithm for SoroNum<i128> {
    fn log2<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env,
    ) -> Result<Self, ArithmeticError> {
        if self.value <= 0 {
            return Err(ArithmeticError::InvalidInput);
        }

        let mut value = I256::from_i128(env, self.value);
        let mut log2_result = I256::from_i128(env, 0);

        // Iterate to calculate log2
        while value > I256::from_i128(env, 1) {
            value = value.div(&I256::from_i128(env, 2));
            log2_result = log2_result.add(&I256::from_i128(env, 1));
        }

        // Scale result
        let scaled_result = log2_result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT));

        if scaled_result > I256::from_i128(env, i128::MAX)
            || scaled_result < I256::from_i128(env, i128::MIN)
        {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum {
                value: scaled_result.to_i128().unwrap(),
                scale: SCALE_OUT,
            })
        }
    }

    fn log10<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env,
    ) -> Result<Self, ArithmeticError> {
        if self.value <= 0 {
            return Err(ArithmeticError::InvalidInput);
        }

        let mut value = I256::from_i128(env, self.value);
        let mut log10_result = I256::from_i128(env, 0);
        let ten = I256::from_i128(env, 10);

        // Iterate to calculate log10
        while value >= ten {
            value = value.div(&ten);
            log10_result = log10_result.add(&I256::from_i128(env, 1));
        }

        // Scale result
        let scaled_result = log10_result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT));

        if scaled_result > I256::from_i128(env, i128::MAX)
            || scaled_result < I256::from_i128(env, i128::MIN)
        {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum {
                value: scaled_result.to_i128().unwrap(),
                scale: SCALE_OUT,
            })
        }
    }
}
