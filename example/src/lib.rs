#![no_std]
use core::ops::Add;

use soroban_math::{CoreArith, SoroNum, pow::Power, log::Logarithm, root::Sqrt, trig::Trigonometry};
use soroban_sdk::{contract, contractimpl, U256, I256, Env};

#[contract]
pub struct SorobanMathExample;

#[contractimpl]
impl SorobanMathExample {
    pub fn simple_u32_add(a: u32, b: u32) -> u32 {
        let x = SoroNum::new(a);
        let y = SoroNum::new(b);
        let m = x.clone().add(y).unwrap();
        *m.value()
    }

    pub fn pow_u128_base_u32_exponent(a: u128, b: u32) -> u128 {
        let x = SoroNum::new(a);
        let m = x.pow(b).unwrap();
        *m.value()
    }

    pub fn pow_i128_base_u32_exponent(a: i128, b: u32) -> i128 {
        let x = SoroNum::new(a);
        let m = x.pow(b).unwrap();
        *m.value()
    }

    pub fn log_2_i128(a: i128) -> u32 {
        let num = SoroNum { value: a };
        num.log2().unwrap()
   
    }

    pub fn root(e: Env, a: U256) -> U256 {
        let num1 = SoroNum { value: a };
        num1.sqrt(&e).value().clone()
    }

    pub fn log_2_u256(a: U256) -> u32 {
        let num = SoroNum { value: a };
        num.log2().unwrap()
    }

    
    pub fn sin_u256(e: Env, a: U256) -> U256 {
        let num = SoroNum { value: a };
        num.sin(&e).value().clone()
    }

    pub fn cos_u256(e: Env, a: U256) -> U256 {
        let num = SoroNum { value: a };
        num.cos(&e).value().clone()
    }

    pub fn sin_i256(e: Env, a: I256) -> I256 {
        let num = SoroNum { value: a };
        num.sin(&e).value().clone()
    }
    pub fn cos_i256(e: Env, a: I256) -> I256 {
        let num = SoroNum { value: a };
        num.cos(&e).value().clone()
    }
    
}
mod test;
