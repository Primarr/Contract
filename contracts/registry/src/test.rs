#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Symbol, Address};

#[test]
fn test_registry() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Registry);
    let client = RegistryClient::new(&env, &contract_id);

    let provider = Address::random(&env);
    let service_id = Symbol::new(&env, "test_service");

    // Test register
    let result = client.register(&service_id, &provider, &1000i128);
    assert!(result);

    // Test get_price
    let price = client.get_price(&service_id);
    assert_eq!(price, 1000i128);
}
