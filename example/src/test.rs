#![cfg(test)]

use super::*;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SorobanMathExample);
    let client = SorobanMathExampleClient::new(&env, &contract_id);
    let sum_u32 = client.test_i128_add();
    assert_eq!(sum_u32, 2469126700);
    let _ = client.test_i128_pow();
    let _ = client.test_i256_pow();

    let val = client.test_log_2_i128();
    assert_eq!(val, 10);
    let _ = client.test_log_10_i128();
    let x = client.test_root();
    assert_eq!(x, 7071067);
    let xss = client.test_div();
    assert_eq!(xss, 25_i128);
    let _cc = client.test_div_i256();

}
