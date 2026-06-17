#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, token};

#[contract]
pub struct Settlement;

#[contractimpl]
impl Settlement {
    /// Execute payment settlement between agents
    pub fn settle(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        asset: Address,
        service_id: String,
    ) -> bool {
        from.require_auth();

        // Validate payment amount
        if amount <= 0 {
            return false;
        }

        // Record transaction
        let tx_key = Symbol::new(&env, &format!("tx:{}", service_id));
        let timestamp = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set(&tx_key, &(from.clone(), to.clone(), amount, timestamp));

        // Transfer funds (simplified - in production would use token contract)
        let client = token::Client::new(&env, &asset);
        if let Ok(_) = client.transfer(&from, &to, &amount) {
            return true;
        }

        false
    }

    /// Get transaction details
    pub fn get_transaction(env: Env, service_id: String) -> Option<(Address, Address, i128, u64)> {
        let tx_key = Symbol::new(&env, &format!("tx:{}", service_id));

        env.storage()
            .persistent()
            .get::<_, (Address, Address, i128, u64)>(&tx_key)
            .ok()
            .flatten()
    }

    /// Compute protocol fee (e.g., 0.2%)
    pub fn compute_fee(amount: i128, fee_bps: i128) -> i128 {
        // fee_bps = basis points (1 BPS = 0.01%)
        (amount * fee_bps) / 10000i128
    }

    /// Get total fees collected
    pub fn get_total_fees(env: Env) -> i128 {
        let key = Symbol::new(&env, "total_fees");

        env.storage()
            .persistent()
            .get::<_, i128>(&key)
            .unwrap_or(0i128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_compute_fee() {
        // 0.2% fee on 1000 = 2
        let fee = Settlement::compute_fee(1000i128, 20i128);
        assert_eq!(fee, 2i128);
    }

    #[test]
    fn test_settlement() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Settlement);
        let client = SettlementClient::new(&env, &contract_id);

        let from = Address::random(&env);
        let to = Address::random(&env);
        let asset = Address::random(&env);
        let service_id = soroban_sdk::String::from_str(&env, "test-service");

        // Note: Full settlement test would require mocking token contract
        let fee = client.compute_fee(&1000i128, &20i128);
        assert_eq!(fee, 2i128);
    }
}
