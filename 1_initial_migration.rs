use ink::prelude::vec::Vec;
use aleph_primitives::{AccountId, Runtime};
use ink::storage::Mapping;

#[ink::contract]
mod migrations {
    use super::*;

    #[ink(storage)]
    pub struct Migrations;

    #[ink(event)]
    pub struct ContractDeployed {
        #[ink(topic)]
        pub contract_address: AccountId,
    }

    impl Migrations {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn deploy_cookie_chain(&mut self) {
            let contract = cookiechain::Cookiechain::new();
            let contract_address = contract.env().account_id();
            self.env().emit_event(ContractDeployed {
                contract_address: contract_address,
            });
        }
    }
}
