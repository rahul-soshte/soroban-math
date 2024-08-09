//! Core Math Functions
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]

use crate::error::ArithmeticError;
use soroban_sdk::{Env, I256};
pub mod error;
pub mod pow;
pub mod log;
// pub mod trig;
// pub mod root;

#[derive(Clone, Copy, Debug, Default)]
pub struct SoroNum<T> {
    pub value: T,
    pub scale: u32,
}


impl<T> SoroNum<T> {
    pub fn new(value: T, scale: u32) -> Self {
        Self { value, scale }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }
}

pub trait CoreArith {
    fn add<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &Self,
        e: &Env
    ) -> Result<Self, ArithmeticError> where Self: Sized;

    fn sub<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &Self,
        e: &Env
    ) -> Result<Self, ArithmeticError> where Self: Sized;

    fn mul<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &Self,
        e: &Env
    ) -> Result<Self, ArithmeticError> where Self: Sized;

    fn div<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &Self,
        e: &Env
    ) -> Result<Self, ArithmeticError> where Self: Sized;
}


impl CoreArith for SoroNum<i128> {

    fn add<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &Self,
        env: &Env
    ) -> Result<Self, ArithmeticError> {
        let self_scaled = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - self.scale));
        let other_scaled = I256::from_i128(env, other.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - other.scale));
        
        let result = self_scaled.add(&other_scaled);
        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT)).div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap(), scale: SCALE_OUT })
        }
    }

    fn sub<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &Self,
        env: &Env
    ) -> Result<Self, ArithmeticError> {
        let self_scaled = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - self.scale));
        let other_scaled = I256::from_i128(env, other.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - other.scale));
        
        let result = self_scaled.sub(&other_scaled);
        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT)).div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Underflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap(), scale: SCALE_OUT })
        }
    }

    fn mul<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &Self,
        env: &Env
    ) -> Result<Self, ArithmeticError> {
        let self_scaled = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - self.scale));
        let other_scaled = I256::from_i128(env, other.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - other.scale));
        
        let result = self_scaled.mul(&other_scaled)
            .div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT)).div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap(), scale: SCALE_OUT })
        }
    }

    fn div<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        other: &Self,
        env: &Env
    ) -> Result<Self, ArithmeticError> {
        if other.value == 0 {
            return Err(ArithmeticError::DivisionByZero);
        }

        let self_scaled = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE * 2 - self.scale));
        let other_scaled = I256::from_i128(env, other.value).mul(&I256::from_i128(env, 10).pow(CALC_SCALE - other.scale));
        
        let result = self_scaled.div(&other_scaled);

        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT)).div(&I256::from_i128(env, 10).pow(CALC_SCALE));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap(), scale: SCALE_OUT })
        }
    }
}
