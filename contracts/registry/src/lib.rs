#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, Map, String, Symbol, Vec, Address};

// Simple key-value storage for services (id -> (capability, provider, price, status))
// Using primitives instead of complex struct for Soroban compatibility

#[contract]
pub struct Registry;

#[contractimpl]
impl Registry {
    /// Register a new service in the registry
    pub fn register(
        env: Env,
        service_id: String,
        capability: String,
        provider: Address,
        price_per_call: i128,
    ) -> bool {
        provider.require_auth();

        // Store service metadata with composite key
        let id_key = Symbol::new(&env, &format!("svc:{}", service_id));

        if env.storage().persistent().has(&id_key) {
            return false;
        }

        // Store as tuple: (capability, provider, price, status=0)
        env.storage()
            .persistent()
            .set(&id_key, &(capability, provider, price_per_call, 0u32));

        true
    }

    /// Get service price by ID
    pub fn get_price(env: Env, service_id: String) -> Option<i128> {
        let id_key = Symbol::new(&env, &format!("svc:{}", service_id));

        if let Some((_, _, price, status)) = env
            .storage()
            .persistent()
            .get::<_, (String, Address, i128, u32)>(&id_key)
            .ok()
            .flatten()
        {
            if status == 0 {
                return Some(price);
            }
        }
        None
    }

    /// Get service provider by ID
    pub fn get_provider(env: Env, service_id: String) -> Option<Address> {
        let id_key = Symbol::new(&env, &format!("svc:{}", service_id));

        if let Some((_, provider, _, status)) = env
            .storage()
            .persistent()
            .get::<_, (String, Address, i128, u32)>(&id_key)
            .ok()
            .flatten()
        {
            if status == 0 {
                return Some(provider);
            }
        }
        None
    }

    /// Update service price
    pub fn update_price(env: Env, service_id: String, new_price: i128) -> bool {
        let id_key = Symbol::new(&env, &format!("svc:{}", service_id));

        if let Some((capability, provider, _, status)) = env
            .storage()
            .persistent()
            .get::<_, (String, Address, i128, u32)>(&id_key)
            .ok()
            .flatten()
        {
            provider.require_auth();
            env.storage()
                .persistent()
                .set(&id_key, &(capability, provider, new_price, status));
            true
        } else {
            false
        }
    }

    /// Set service status (0=active, 1=paused, 2=deprecated)
    pub fn set_status(env: Env, service_id: String, new_status: u32) -> bool {
        let id_key = Symbol::new(&env, &format!("svc:{}", service_id));

        if let Some((capability, provider, price, _)) = env
            .storage()
            .persistent()
            .get::<_, (String, Address, i128, u32)>(&id_key)
            .ok()
            .flatten()
        {
            provider.require_auth();
            env.storage()
                .persistent()
                .set(&id_key, &(capability, provider, price, new_status));
            true
        } else {
            false
        }
    }
}

mod test;
