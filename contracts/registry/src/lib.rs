#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct Registry;

#[contractimpl]
impl Registry {
    /// Register a new service in the registry
    pub fn register(env: Env, service_id: Symbol, provider: Address, price_per_call: i128) -> bool {
        provider.require_auth();

        if env.storage().persistent().has(&service_id) {
            return false;
        }

        // Store as tuple: (provider, price, status=0)
        env.storage()
            .persistent()
            .set(&service_id, &(provider.clone(), price_per_call, 0u32));

        true
    }

    /// Get service price by ID
    pub fn get_price(env: Env, service_id: Symbol) -> i128 {
        if let Some((_, price, status)) = env
            .storage()
            .persistent()
            .get::<_, (Address, i128, u32)>(&service_id)
        {
            if status == 0 {
                return price;
            }
        }
        0i128
    }

    /// Update service price
    pub fn update_price(env: Env, service_id: Symbol, new_price: i128) -> bool {
        if let Some((provider, _, status)) = env
            .storage()
            .persistent()
            .get::<_, (Address, i128, u32)>(&service_id)
        {
            provider.require_auth();
            env.storage()
                .persistent()
                .set(&service_id, &(provider, new_price, status));
            return true;
        }
        false
    }

    /// Set service status (0=active, 1=paused, 2=deprecated)
    pub fn set_status(env: Env, service_id: Symbol, new_status: u32) -> bool {
        if let Some((provider, price, _)) = env
            .storage()
            .persistent()
            .get::<_, (Address, i128, u32)>(&service_id)
        {
            provider.require_auth();
            env.storage()
                .persistent()
                .set(&service_id, &(provider, price, new_status));
            return true;
        }
        false
    }
}

mod test;
