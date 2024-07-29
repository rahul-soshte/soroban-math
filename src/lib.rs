//! Core Math Functions
#![cfg_attr(not(feature = "std"), no_std)]

use crate::error::ArithmeticError;
use soroban_sdk::{Env, I256};
pub mod error;
// pub mod pow;
// pub mod log;
// pub mod trig;
// pub mod root;

#[derive(Clone, Copy, Debug, Default)]
pub struct SoroNum<T, const SCALE: u32> {
    pub value: T,
}

impl<T, const SCALE: u32> SoroNum<T, SCALE> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn scale() -> u32 {
        SCALE
    }
}

pub trait CoreArith {
    fn add<const SCALE_IN: u32, const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &SoroNum<Self::Inner, SCALE_IN>,
        e: &Env
    ) -> Result<SoroNum<Self::Inner, SCALE_OUT>, ArithmeticError>;

    fn sub<const SCALE_IN: u32, const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &SoroNum<Self::Inner, SCALE_IN>,
        e: &Env
    ) -> Result<SoroNum<Self::Inner, SCALE_OUT>, ArithmeticError>;

    fn mul<const SCALE_IN: u32, const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &SoroNum<Self::Inner, SCALE_IN>,
        e: &Env
    ) -> Result<SoroNum<Self::Inner, SCALE_OUT>, ArithmeticError>;

    fn div<const SCALE_IN: u32, const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &SoroNum<Self::Inner, SCALE_IN>,
        e: &Env

    ) -> Result<SoroNum<Self::Inner, SCALE_OUT>, ArithmeticError>;

    type Inner;
}


impl<const SCALE: u32> CoreArith for SoroNum<i128, SCALE> {
    type Inner = i128;

    fn add<const SCALE_IN: u32, const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &SoroNum<i128, SCALE_IN>,
        env: &Env
    ) -> Result<SoroNum<i128, SCALE_OUT>, ArithmeticError> {
        let self_scaled = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - SCALE));
        let other_scaled = I256::from_i128(env, other.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - SCALE_IN));
        
        let result = self_scaled.add(&other_scaled);
        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT)).div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap() })
        }
    }

    fn sub<const SCALE_IN: u32, const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &SoroNum<i128, SCALE_IN>,
        env: &Env
    ) -> Result<SoroNum<i128, SCALE_OUT>, ArithmeticError> {
        let self_scaled = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - SCALE));
        let other_scaled = I256::from_i128(env, other.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - SCALE_IN));
        
        let result = self_scaled.sub(&other_scaled);
        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT)).div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Underflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap() })
        }
    }

    fn mul<const SCALE_IN: u32, const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &SoroNum<i128, SCALE_IN>,
        env: &Env
    ) -> Result<SoroNum<i128, SCALE_OUT>, ArithmeticError> {
        let self_scaled = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - SCALE));
        let other_scaled = I256::from_i128(env, other.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - SCALE_IN));
        
        let result = self_scaled.mul(&other_scaled)
            .div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT)).div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap() })
        }
    }

    fn div<const SCALE_IN: u32, const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &SoroNum<i128, SCALE_IN>,
        env: &Env
    ) -> Result<SoroNum<i128, SCALE_OUT>, ArithmeticError> {
        if other.value == 0 {
            return Err(ArithmeticError::DivisionByZero);
        }

        let self_scaled = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE * 2 - SCALE));
        let other_scaled = I256::from_i128(env, other.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - SCALE_IN));
        
        let result = self_scaled.div(&other_scaled);

        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT)).div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap() })
        }
    }
}
