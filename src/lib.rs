//! Core Math Functions
#![cfg_attr(not(feature = "std"), no_std)]

use crate::error::ArithmeticError;
use soroban_sdk::{Env, I256, U256};
pub mod root;
pub mod error;
pub mod pow;
pub mod log;

#[derive(Clone, Copy, Debug, Default)]
pub struct SoroNum<T> {
    pub value: T,
}

impl<T> SoroNum<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

pub trait CoreArith {
    fn add(self, other: Self) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
    fn sub(self, other: Self) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
    fn mul(self, other: Self) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
}

impl CoreArith for SoroNum<i32> {
    fn add(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_add(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_sub(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Underflow)
    }

    fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_mul(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
        if other.value == 0 {
            Err(ArithmeticError::DivisionByZero)
        } else {
            self.value
                .checked_div(other.value)
                .map(|value| SoroNum { value })
                .ok_or(ArithmeticError::DivisionByZero)
        }
    }
}

impl CoreArith for SoroNum<u32> {
    fn add(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_add(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_sub(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Underflow)
    }

    fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_mul(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
        if other.value == 0 {
            Err(ArithmeticError::DivisionByZero)
        } else {
            self.value
                .checked_div(other.value)
                .map(|value| SoroNum { value })
                .ok_or(ArithmeticError::DivisionByZero)
        }
    }
}

impl CoreArith for SoroNum<u64> {
    fn add(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_add(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_sub(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Underflow)
    }

    fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_mul(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
        if other.value == 0 {
            Err(ArithmeticError::DivisionByZero)
        } else {
            self.value
                .checked_div(other.value)
                .map(|value| SoroNum { value })
                .ok_or(ArithmeticError::DivisionByZero)
        }
    }
}

impl CoreArith for SoroNum<i64> {
    fn add(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_add(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_sub(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Underflow)
    }

    fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_mul(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
        if other.value == 0 {
            Err(ArithmeticError::DivisionByZero)
        } else {
            self.value
                .checked_div(other.value)
                .map(|value| SoroNum { value })
                .ok_or(ArithmeticError::DivisionByZero)
        }
    }
}

impl CoreArith for SoroNum<i128> {
    fn add(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_add(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_sub(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Underflow)
    }

    fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_mul(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
        if other.value == 0 {
            Err(ArithmeticError::DivisionByZero)
        } else {
            self.value
                .checked_div(other.value)
                .map(|value| SoroNum { value })
                .ok_or(ArithmeticError::DivisionByZero)
        }
    }
}

// u128
impl CoreArith for SoroNum<u128> {
    fn add(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_add(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_sub(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Underflow)
    }

    fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value
            .checked_mul(other.value)
            .map(|value| SoroNum { value })
            .ok_or(ArithmeticError::Overflow)
    }

    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
        if other.value == 0 {
            Err(ArithmeticError::DivisionByZero)
        } else {
            self.value
                .checked_div(other.value)
                .map(|value| SoroNum { value })
                .ok_or(ArithmeticError::DivisionByZero)
        }
    }
}

impl CoreArith for SoroNum<U256> {
    fn add(self, other: Self) -> Result<Self, ArithmeticError> {
        Ok(SoroNum {
            value: self.value.add(&other.value),
        })
    }
    fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
        Ok(SoroNum {
            value: self.value.sub(&other.value),
        })
    }
    fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
        Ok(SoroNum {
            value: self.value.mul(&other.value),
        })
    }
    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
        if other.value == U256::from_u32(env, 0) {
            Err(ArithmeticError::DivisionByZero)
        } else {
            Ok(SoroNum {
                value: self.value.div(&other.value),
            })
        }
    }
}

impl CoreArith for SoroNum<I256> {
    fn add(self, other: Self) -> Result<Self, ArithmeticError> {
        Ok(SoroNum {
            value: self.value.add(&other.value),
        })
    }
    fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
        Ok(SoroNum {
            value: self.value.sub(&other.value),
        })
    }
    fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
        Ok(SoroNum {
            value: self.value.mul(&other.value),
        })
    }
    fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
        if other.value == I256::from_i32(&env, 0) {
            Err(ArithmeticError::DivisionByZero)
        } else {
            Ok(SoroNum {
                value: self.value.div(&other.value),
            })
        }
    }
}