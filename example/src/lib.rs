#![no_std]
use soroban_math::root::Root;
use soroban_math::SoroResult;
use soroban_math::{log::Logarithm, pow::Power, CoreArith, SoroNum};
use soroban_sdk::{contract, contractimpl, Env};
use soroban_sdk::I256;

#[contract]
pub struct SorobanMathExample;

#[contractimpl]
impl SorobanMathExample {
    pub fn test_i128_add(e: &Env) -> i128 {
        let a = SoroNum::<i128>::new(1_234_567, 6); // 1.234567
        let b = SoroNum::<i128>::new(23_4567, 4); // 23.4567

        let m = a.add::<20, 8>(&b, e, false).unwrap();
        let val = match m {
            SoroResult::I128(soronum) => soronum,
            _ => panic!("Invalid type"),
        };

        *val.value()
    }

    pub fn test_i128_pow(e: &Env) {
        let base = SoroNum::new(2_i128, 0); // scale 0 for integers
        let result = base.pow::<10, 0>(3, &e).unwrap();
        assert_eq!(result.value, 8);
    }

    pub fn test_log_2_i128(e: &Env) -> i128 {
        const CALC_SCALE: u32 = 18; // This is an assumption, adjust if different
        const SCALE_OUT: u32 = 0;
        let value = SoroNum {
            value: 1024_i128,
            scale: 0,
        }; // 2^10 = 1024
        let result = value.log2::<CALC_SCALE, SCALE_OUT>(e);
        assert!(result.is_ok());
        let result_value = result.unwrap();
        return *result_value.value();
    }

    pub fn test_log_10_i128(e: &Env) {
        // Input: 1000
        let input_num = SoroNum {
            value: 1000,
            scale: 0,
        };
        let result = input_num.log10::<18, 1>(e);
        let expected_value = 30;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, expected_value);
    }

    pub fn test_root(e: &Env) -> i128 {
        let number = SoroNum::new(50000000, 6); // sqrt(100) should be 10.0
        let result = number.sqrt::<12, 6>(e).unwrap();
        // assert_eq!(result.value, 10000000);
        return *result.value();
    }


    pub fn test_div(e: &Env) -> i128 {
        let number1 = SoroNum::new(50, 0); 
        let number2 = SoroNum::new(2, 0); 

        let result = number1.div::<10, 0>(&number2, e, false).unwrap();
        let val = match result {
            SoroResult::I128(soronum) => soronum,
            _ => panic!("Invalid type"),
        };
       
        return *val.value();
    }

    pub fn test_div_i256(e: &Env) {
        let number1 = SoroNum::new(I256::from_i32(e, 100), 0); 
        let number2 = SoroNum::new(I256::from_i32(e, 2), 0); 

        let result = number1.div::<10, 0>(&number2, e, true).unwrap();
        let val = match result {
            SoroResult::I256(soronum) => soronum,
            _ => panic!("Invalid type"),
        };
       
        assert_eq!(*val.value(), I256::from_i32(e, 50_i32));
        //TODO: Need to able to return custom type I think, as I256 should also be able to returned
        // return *val.value();
    }
}

mod test;
