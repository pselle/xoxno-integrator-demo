use super::*;
use soroban_sdk::{Address, testutils::Address as _};
use stellar_registry::testutils::register_import;

fn client(env: &Env) -> HealthCheckClient<'_> {
    HealthCheckClient::new(env, &env.register(HealthCheck, ()))
}

#[test]
fn reads_the_real_controller_wasm() {
    let env = Env::default();
    let admin = Address::generate(&env);
    register_import!(&env, "xoxno/controller", args = { admin });

    let hc = client(&env);
    assert!(hc.controller_version() > 0);
    assert_eq!(hc.health(&1), Health::NoAccount);
}

/// Stands in for the controller to put accounts in each state.
#[contract]
struct MockController;

const WAD: i128 = 1_000_000_000_000_000_000;

#[contractimpl]
impl MockController {
    pub fn account_exists(_env: &Env, account_id: u64) -> bool {
        account_id != 0
    }
    pub fn get_health_factor(_env: &Env, account_id: u64) -> i128 {
        if account_id == 1 { 2 * WAD } else { WAD / 2 }
    }
    pub fn is_liquidatable(_env: &Env, account_id: u64) -> bool {
        account_id != 1
    }
}

#[test]
fn classifies_accounts() {
    let env = Env::default();
    register_import!(&env, "xoxno/controller", contract = MockController);

    let hc = client(&env);
    assert_eq!(hc.health(&0), Health::NoAccount);
    assert_eq!(hc.health(&1), Health::Healthy(2 * WAD));
    assert_eq!(hc.health(&2), Health::Liquidatable(WAD / 2));
}
