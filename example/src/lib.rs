#![no_std]
use core::ops::Add;

use soroban_math::{CoreArith, SoroNum};
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
}

mod test;
