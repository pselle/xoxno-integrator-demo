#![no_std]
//! An integrator's view of XOXNO lending: reads an account's health from the
//! `xoxno/controller` registered in the Stellar Registry.
use soroban_sdk::{Env, contract, contractimpl, contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Health {
    NoAccount,
    Healthy(i128),
    Liquidatable(i128),
}

#[contract]
pub struct HealthCheck;

#[contractimpl]
impl HealthCheck {
    /// The controller's app version, as a smoke test of the import.
    pub fn controller_version(env: &Env) -> u32 {
        stellar_registry::import_contract!(env, "xoxno/controller").get_app_version()
    }

    /// `account_id`'s health factor (WAD), and whether it can be liquidated.
    pub fn health(env: &Env, account_id: u64) -> Health {
        let controller = stellar_registry::import_contract!(env, "xoxno/controller");
        if !controller.account_exists(&account_id) {
            return Health::NoAccount;
        }
        let hf = controller.get_health_factor(&account_id);
        if controller.is_liquidatable(&account_id) {
            Health::Liquidatable(hf)
        } else {
            Health::Healthy(hf)
        }
    }
}

#[cfg(test)]
mod test;
