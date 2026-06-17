#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Budget;

#[contractimpl]
impl Budget {
    /// Configure spending limits for an agent
    pub fn set_limit(env: Env, agent: Symbol, session_cap: i128, task_cap: i128) -> bool {
        env.storage()
            .persistent()
            .set(&agent, &(session_cap, task_cap));

        true
    }

    /// Check if payment would exceed budget
    pub fn check_limit(
        env: Env,
        agent: Symbol,
        amount: i128,
        payment_type: u32, // 0 = session, 1 = task
    ) -> bool {
        if let Some((session_cap, task_cap)) =
            env.storage().persistent().get::<_, (i128, i128)>(&agent)
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
    pub fn get_limit(env: Env, agent: Symbol) -> Option<(i128, i128)> {
        env.storage().persistent().get::<_, (i128, i128)>(&agent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Symbol};

    #[test]
    fn test_budget() {
        let env = Env::default();
        let contract_id = env.register(Budget, ());
        let client = BudgetClient::new(&env, &contract_id);

        let agent = Symbol::new(&env, "test_agent");

        // Set limit
        let result = client.set_limit(&agent, &5000i128, &1000i128);
        assert!(result);

        // Check limit
        let check = client.check_limit(&agent, &2000i128, &0u32);
        assert!(check);
    }
}
