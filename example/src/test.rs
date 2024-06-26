#![cfg(test)]


use super::*;
use soroban_sdk::{Env, U256, I256};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SorobanMathExample);
    let client = SorobanMathExampleClient::new(&env, &contract_id);

    let sum_u32 = client.simple_u32_add(&34, &56);
    assert_eq!(sum_u32, 90);

    let pow_u128_u32 = client.pow_u128_base_u32_exponent(&5_u128, &2_u32);
    assert_eq!(pow_u128_u32, 25_u128);

    let pow_i128_u32 = client.pow_i128_base_u32_exponent(&5_i128, &2_u32);
    assert_eq!(pow_i128_u32, 25_i128);

    let log2_u128 = client.log_2_i128(&1024);
    assert_eq!(log2_u128, 10_u32);

    //TODO: Case where the extra decimal part is cut off. base Precision stuff.
    let root = client.root(&U256::from_u32(&env, 16_u32));
    assert_eq!(root, U256::from_u32(&env, 4_u32));

    let log2_u128 = client.log_2_u256(&U256::from_u128(&env, 1024));
    assert_eq!(log2_u128, 10_u32);
    
    //TODO: Do proper scaled division for improvement in output
    let sin_u256 = client.sin_u256(&U256::from_u128(&env, 60));
    assert_eq!(sin_u256, U256::from_u128(&env, 866021));

    let cos_u256 = client.cos_u256(&U256::from_u128(&env, 60));
    assert_eq!(cos_u256, U256::from_u128(&env, 499966));

    let sin_i256 = client.sin_i256(&I256::from_i128(&env, 60));
    assert_eq!(sin_i256, I256::from_i128(&env, 866021));

    let cos_i256 = client.cos_i256(&I256::from_i128(&env, 60));
    assert_eq!(cos_i256, I256::from_i128(&env, 499966));
}
