use soroban_sdk::{I256, Env, Bytes};

use crate::{SoroNum, error::ArithmeticError};
pub trait Logarithm {
    fn log2<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env
    ) -> Result<Self, ArithmeticError> where Self: Sized;

    fn log10<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env
    ) -> Result<Self, ArithmeticError> where Self: Sized;
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
        env: &Env
    ) -> Result<Self, ArithmeticError> {
        if self.value <= 0 {
            return Err(ArithmeticError::InvalidInput);
        }

        let scaled_value = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - self.scale));
        let log2 = I256::from_i128(env, 255 - scaled_value.leading_zeros() as i128);
        let scaled_result = log2.mul(&I256::from_i128(env, 10).pow(SCALE_OUT));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap(), scale: SCALE_OUT })
        }
    }

    fn log10<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env
    ) -> Result<Self, ArithmeticError> {
        if self.value <= 0 {
            return Err(ArithmeticError::InvalidInput);
        }

        let scaled_value = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - self.scale));
        let mut count = I256::from_i128(env, 0);
        let mut num = scaled_value;
        let ten = I256::from_i128(env, 10);
        
        while num >= ten.pow(CALC_SCALE) {
            num = num.div(&ten);
            count = count.add(&I256::from_i128(env, 1));
        }

        let scaled_result = count.mul(&I256::from_i128(env, 10).pow(SCALE_OUT));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap(), scale: SCALE_OUT })
        }
    }
}

