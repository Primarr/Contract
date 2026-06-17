#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct Settlement;

#[contractimpl]
impl Settlement {
    /// Record a payment settlement between agents
    pub fn record_settlement(
        env: Env,
        tx_id: Symbol,
        from: Address,
        to: Address,
        amount: i128,
    ) -> bool {
        from.require_auth();

        if amount <= 0 {
            return false;
        }

        let timestamp = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&tx_id, &(from, to, amount, timestamp));

        true
    }

    /// Get transaction details
    pub fn get_transaction(env: Env, tx_id: Symbol) -> Option<(Address, Address, i128, u64)> {
        env.storage()
            .persistent()
            .get::<_, (Address, Address, i128, u64)>(&tx_id)
    }

    /// Compute protocol fee (basis points)
    pub fn compute_fee(amount: i128, fee_bps: i128) -> i128 {
        // fee_bps = basis points (20 BPS = 0.2%)
        (amount * fee_bps) / 10000i128
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fee() {
        // 0.2% fee (20 BPS) on 1000 = 2
        let fee = Settlement::compute_fee(1000i128, 20i128);
        assert_eq!(fee, 2i128);
    }
}
