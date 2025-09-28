#![cfg(test)]

use soroban_sdk::{vec, Env, String, Vec, Address};

use crate::{Contract, ContractClient};

#[test]
fn test() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(Contract, ());
    let client: ContractClient<'_> = ContractClient::new(&env, &contract_id);

    let words: Vec<String> = client.hello(&String::from_str(&env, "Dev"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Dev"),
        ]
    );
}