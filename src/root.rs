use soroban_sdk::{ Env, I256};
use crate::{error::ArithmeticError, SoroNum};


pub trait Root {
    fn sqrt<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env
    ) -> Result<Self, ArithmeticError> where Self: Sized;
}

impl Root for SoroNum<i128> {
    fn sqrt<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        env: &Env
    ) -> Result<Self, ArithmeticError> {
        if self.value < 0 {
            return Err(ArithmeticError::DomainError); // Square root of a negative number is not defined
        }
        if self.value == 0 {
            return Ok(SoroNum::new(0, SCALE_OUT)); // Square root of 0 is 0
        }

        let two = I256::from_i128(env, 2);
        let ten_pow_calc_scale = I256::from_i128(env, 10).pow(CALC_SCALE - SCALE_OUT);
        let ten_pow_2_calc_scale = I256::from_i128(env, 10).pow((CALC_SCALE - SCALE_OUT) * 2);
        
        // Initial guess for Newton's method
        let mut x = I256::from_i128(env, self.value)
            .mul(&ten_pow_calc_scale)
            .div(&two);

        // Newton's iteration
        for _ in 0..30 { // Limit to a reasonable number of iterations
            let x_old = x.clone();
            let temp = I256::from_i128(env, self.value)
                .mul(&ten_pow_2_calc_scale)
                .div(&x);
            x = x.add(&temp).div(&two);

            if x == x_old { // Convergence check
                break;
            }
        }

        // Calculate the scale factor for the final result
        let scale_factor = I256::from_i128(env, 10).pow((CALC_SCALE - SCALE_OUT)/2);
        let scaled_result = x.div(&scale_factor);

        // Check for overflow and return the result
        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap(), scale: SCALE_OUT })
        }
    }
}


