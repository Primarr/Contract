#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct Budget;

#[contractimpl]
impl Budget {
    /// Configure spending limits for an agent
    pub fn set_limit(
        env: Env,
        agent: Address,
        session_cap: i128,
        task_cap: i128,
        rate_limit: u32,
    ) -> bool {
        agent.require_auth();

        let key = Symbol::new(&env, &format!("limit:{}", agent));
        env.storage()
            .persistent()
            .set(&key, &(session_cap, task_cap, rate_limit));

        true
    }

    /// Check if payment would exceed budget
    pub fn check_limit(
        env: Env,
        agent: Address,
        amount: i128,
        payment_type: u32, // 0 = session, 1 = task
    ) -> bool {
        let key = Symbol::new(&env, &format!("limit:{}", agent));

        if let Some((session_cap, task_cap, _)) = env
            .storage()
            .persistent()
            .get::<_, (i128, i128, u32)>(&key)
            .ok()
            .flatten()
        {
            match payment_type {
                0 => amount <= session_cap,
                1 => amount <= task_cap,
                _ => false,
            }
        } else {
            true // No limit set, allow
        }
    }

    /// Get current budget for agent
    pub fn get_limit(env: Env, agent: Address) -> Option<(i128, i128, u32)> {
        let key = Symbol::new(&env, &format!("limit:{}", agent));

        env.storage()
            .persistent()
            .get::<_, (i128, i128, u32)>(&key)
            .ok()
            .flatten()
    }

    /// Reset session budget
    pub fn reset_session(env: Env, agent: Address) -> bool {
        agent.require_auth();

        let key = Symbol::new(&env, &format!("spent:{}", agent));
        env.storage().persistent().remove(&key);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_budget() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Budget);
        let client = BudgetClient::new(&env, &contract_id);

        let agent = Address::random(&env);

        // Set limit
        let result = client.set_limit(&agent, &5000i128, &1000i128, &100u32);
        assert!(result);

        // Check limit
        let check = client.check_limit(&agent, &2000i128, &0u32);
        assert!(check);
    }
}
