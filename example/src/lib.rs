#![no_std]
use core::ops::Add;

use soroban_math::{CoreArith, SoroNum, pow::Power, log::Logarithm};
use soroban_sdk::{contract, contractimpl, Env};

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
}

mod test;
