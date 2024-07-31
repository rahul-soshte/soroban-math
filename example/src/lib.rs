#![no_std]
use soroban_math::{CoreArith, SoroNum, pow::Power};
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SorobanMathExample;

#[contractimpl]
impl SorobanMathExample {

    pub fn test_i128_add(e: &Env) -> i128 {
        let a = SoroNum::<i128>::new(1_234_567, 6);  // 1.234567
        let b = SoroNum::<i128>::new(23_4567, 4);    // 23.4567
        let m: SoroNum<i128> = a.add::<20, 8>(&b, e).unwrap();
        *m.value()
    }

    pub fn test_i128_pow(e: &Env) {
        let base = SoroNum::new(2_i128, 0);  // scale 0 for integers
        let result = base.pow::<10, 0>(3, &e).unwrap();
        assert_eq!(result.value, 8);
    }

    // pub fn simple_u32_add(a: u32, b: u32) -> u32 {
    //     let x = SoroNum::new(a);
    //     let y = SoroNum::new(b);
    //     let m = x.clone().add(y).unwrap();
    //     *m.value()
    // }

    // pub fn pow_u128_base_u32_exponent(a: u128, b: u32) -> u128 {
    //     let x = SoroNum::new(a);
    //     let m = x.pow(b).unwrap();
    //     *m.value()
    // }

    // pub fn pow_i128_base_u32_exponent(a: i128, b: u32) -> i128 {
    //     let x = SoroNum::new(a);
    //     let m = x.pow(b).unwrap();
    //     *m.value()
    // }

    // pub fn log_2_i128(a: i128) -> u32 {
    //     let num = SoroNum { value: a };
    //     num.log2().unwrap()
   
    // }

    // pub fn root(e: Env, a: U256) -> U256 {
    //     let num1 = SoroNum { value: a };
    //     num1.sqrt(&e).value().clone()
    // }

    // pub fn log_2_u256(a: U256) -> u32 {
    //     let num = SoroNum { value: a };
    //     num.log2().unwrap()
    // }

    
    // pub fn sin_u256(e: Env, a: U256) -> U256 {
    //     let num = SoroNum { value: a };
    //     num.sin(&e).value().clone()
    // }

    // pub fn cos_u256(e: Env, a: U256) -> U256 {
    //     let num = SoroNum { value: a };
    //     num.cos(&e).value().clone()
    // }

    // pub fn sin_i256(e: Env, a: I256) -> I256 {
    //     let num = SoroNum { value: a };
    //     num.sin(&e).value().clone()
    // }
    // pub fn cos_i256(e: Env, a: I256) -> I256 {
    //     let num = SoroNum { value: a };
    //     num.cos(&e).value().clone()
    // }
    
}
mod test;
