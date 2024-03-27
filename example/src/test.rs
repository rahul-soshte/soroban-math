#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SorobanMathExample);
    let client = SorobanMathExampleClient::new(&env, &contract_id);

    let sum_u32 = client.simple_u32_add(&34, &56);
    assert_eq!(sum_u32, 90)
}