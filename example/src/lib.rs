#![no_std]
use core::ops::Add;

use soroban_math::{SoroNum, CoreArith};
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SorobanMathExample;

#[contractimpl]
impl SorobanMathExample {
    pub fn simple_u32_add(env: Env, a: u32, b: u32) -> u32 {
        let x = SoroNum::new(a);
        let y = SoroNum::new(b);
        let m = x.clone().add(y);
        *m.value()
    }
}

mod test;

