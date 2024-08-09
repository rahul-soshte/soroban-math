#![no_std]
use soroban_math::{log::Logarithm, pow::Power, CoreArith, SoroNum};
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

    
    pub fn test_log_2_i128(e: &Env) -> i128 {
        const CALC_SCALE: u32 = 18; // This is an assumption, adjust if different
        const SCALE_OUT: u32 = 0; 
        let value = SoroNum { value: 1024_i128, scale: 0 }; // 2^10 = 1024
        let result = value.log2::<CALC_SCALE,SCALE_OUT>(e);
        assert!(result.is_ok());
        let result_value = result.unwrap();
        return *result_value.value()

    }

    pub fn test_log_10_i128(e: &Env)  {
        // Input: 1000
        let input_num = SoroNum { value: 1000, scale: 0 };
        let result = input_num.log10::<18, 1>(e);
        let expected_value = 30;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, expected_value);
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
