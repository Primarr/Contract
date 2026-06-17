#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Address};

#[test]
fn test_registry() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Registry);
    let client = RegistryClient::new(&env, &contract_id);

    let provider = Address::random(&env);
    let service_id = String::from_str(&env, "test-service");
    let capability = String::from_str(&env, "inference");

    // Test register
    let result = client.register(&provider, &service_id, &capability, &provider, &1000i128);
    assert!(result);

    // Test get_price
    let price = client.get_price(&service_id);
    assert_eq!(price, Some(1000i128));

    // Test get_provider
    let prov = client.get_provider(&service_id);
    assert!(prov.is_some());
}
