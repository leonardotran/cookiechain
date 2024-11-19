#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod cookie_contract {
    use ink::{
        prelude::string::String,
        prelude::vec::Vec,
        storage::Mapping,
    };

    /// Custom error type for the contract
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        CookieAlreadyExists,
        CookieNotFound,
        NotAuthorized,
        InkEnvError(String),
        InvalidKey,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct CookieEntry {
        profile: String,
        cookie: String,
        expiration_date: String,
        name: String,
        secure: String,
        path: String,
        value: String,
        created_at: BlockNumber,
        owner: AccountId,
        transaction_id: u64,  // Added field for transaction ID
    }

    #[ink(event)]
    pub struct CookieRegistered {
        cookie: String,
        owner: AccountId,
        block: BlockNumber,
        transaction_id: u64,
    }

    #[ink(event)]
    pub struct CookieUpdated {
        cookie: String,
        owner: AccountId,
        block: BlockNumber,
        transaction_id: u64,
    }

    #[ink(event)]
    pub struct CookieDeleted {
        cookie: String,
        owner: AccountId,
        block: BlockNumber,
        transaction_id: u64,
    }

    #[ink(storage)]
    pub struct CookieContract {
        owner: AccountId,
        cookie_count: u32,
        cookies: Mapping<String, CookieEntry>,
        cookie_list: Vec<String>,
        owned_cookies: Mapping<AccountId, Vec<String>>,
        transaction_id_counter: u64,  // Counter for transaction IDs
        public_keys: Mapping<AccountId, String>,  // Store public keys of users
        last_stages: Mapping<AccountId, String>,  // Last stage (profile, domain, etc.)
    }

    impl CookieContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                cookie_count: 0,
                cookies: Mapping::default(),
                cookie_list: Vec::new(),
                owned_cookies: Mapping::default(),
                transaction_id_counter: 0,
                public_keys: Mapping::default(),
                last_stages: Mapping::default(),
            }
        }

        // Register a new cookie with transaction ID
        #[ink(message)]
        pub fn register_cookie(
            &mut self,
            profile: String,
            cookie: String,
            expiration_date: String,
            name: String,
            secure: String,
            path: String,
            value: String,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            let current_block = self.env().block_number();

            // Check if cookie already exists - duplicate
            if self.cookies.contains(&cookie) {
                return Err(Error::CookieAlreadyExists);
            }

            // Generate transaction ID for the current operation
            self.transaction_id_counter = self.transaction_id_counter.wrapping_add(1);
            let transaction_id = self.transaction_id_counter;

            let cookie_entry = CookieEntry {
                profile,
                cookie: cookie.clone(),
                expiration_date,
                name,
                secure,
                path,
                value,
                created_at: current_block,
                owner: caller,
                transaction_id,
            };

            // Update storage
            self.cookies.insert(&cookie, &cookie_entry);
            self.cookie_list.push(cookie.clone());
            self.cookie_count = self.cookie_count.wrapping_add(1);

            // Update owned cookies
            let mut owned = self.owned_cookies.get(caller).unwrap_or_default();
            owned.push(cookie.clone());
            self.owned_cookies.insert(caller, &owned);

            // Emit event
            self.env().emit_event(CookieRegistered {
                cookie,
                owner: caller,
                block: current_block,
                transaction_id,
            });

            Ok(())
        }

        // Incremental update of an existing cookie
        #[ink(message)]
        pub fn update_cookie(
            &mut self,
            profile: String,
            cookie: String,
            expiration_date: String,
            name: String,
            secure: String,
            path: String,
            value: String,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            let current_block = self.env().block_number();

            // Check if cookie exists and caller is owner
            let mut entry = self.cookies.get(&cookie).ok_or(Error::CookieNotFound)?;

            if entry.owner != caller {
                return Err(Error::NotAuthorized);
            }

            // Increment transaction ID for each update
            self.transaction_id_counter = self.transaction_id_counter.wrapping_add(1);
            let transaction_id = self.transaction_id_counter;

            // Create new entry with updated values and new transaction ID
            let cookie_entry = CookieEntry {
                profile,
                cookie: cookie.clone(),
                expiration_date,
                name,
                secure,
                path,
                value,
                created_at: entry.created_at,  // Retain original creation time
                owner: caller,
                transaction_id,
            };

            // Update storage
            self.cookies.insert(&cookie, &cookie_entry);

            // Emit event
            self.env().emit_event(CookieUpdated {
                cookie,
                owner: caller,
                block: current_block,
                transaction_id,
            });

            Ok(())
        }

        // Set or update public key for a user
        #[ink(message)]
        pub fn set_public_key(&mut self, public_key: String) -> Result<(), Error> {
            let caller = self.env().caller();
            self.public_keys.insert(caller, &public_key);
            Ok(())
        }

        // Get a user's public key
        #[ink(message)]
        pub fn get_public_key(&self, account: AccountId) -> Option<String> {
            self.public_keys.get(account)
        }

        // Set or update the last stage for the user (profile, domain, etc.)
        #[ink(message)]
        pub fn set_last_stage(&mut self, stage: String) -> Result<(), Error> {
            let caller = self.env().caller();
            self.last_stages.insert(caller, &stage);
            Ok(())
        }

        // Get the last stage associated with the user
        #[ink(message)]
        pub fn get_last_stage(&self) -> Option<String> {
            let caller = self.env().caller();
            self.last_stages.get(caller)
        }

        // Get cookie details
        #[ink(message)]
        pub fn get_cookie(&self, cookie: String) -> Option<CookieEntry> {
            self.cookies.get(&cookie)
        }

        // Get all cookies
        #[ink(message)]
        pub fn get_all_cookies(&self) -> Vec<String> {
            self.cookie_list.clone()
        }

        // Get cookies owned by an account
        #[ink(message)]
        pub fn get_owned_cookies(&self, account: AccountId) -> Vec<String> {
            self.owned_cookies.get(account).unwrap_or_default()
        }

        // Get total number of cookies
        #[ink(message)]
        pub fn get_cookie_count(&self) -> u32 {
            self.cookie_count
        }

        // Get contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    //***************TEST CASES***************//
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn register_cookie_works() {
            let mut contract = CookieContract::new();
            assert_eq!(
                contract.register_cookie(
                    String::from("profile1"),
                    String::from("cookie1"),
                    String::from("2024-12-31"),
                    String::from("name1"),
                    String::from("secure1"),
                    String::from("/path1"),
                    String::from("value1"),
                ),
                Ok(())
            );
            assert_eq!(contract.get_cookie_count(), 1);
        }

        #[ink::test]
        fn update_cookie_works() {
            let mut contract = CookieContract::new();
            let _ = contract.register_cookie(
                String::from("profile1"),
                String::from("cookie1"),
                String::from("2024-12-31"),
                String::from("name1"),
                String::from("secure1"),
                String::from("/path1"),
                String::from("value1"),
            );
            assert_eq!(
                contract.update_cookie(
                    String::from("profile2"),
                    String::from("cookie1"),
                    String::from("2025-12-31"),
                    String::from("name2"),
                    String::from("secure2"),
                    String::from("/path2"),
                    String::from("value2"),
                ),
                Ok(())
            );
        }

        // #[ink::test]
        // fn public_key_management() {
        //     let mut contract = CookieContract::new();

        //     // Set and get public key
        //     assert_eq!(contract.set_public_key(String::from("user_public_key")), Ok(()));
        //     assert_eq!(contract.get_public_key(contract.env().caller()), Some(String::from("user_public_key")));
        // }

        // #[ink::test]
        // fn last_stage_management() {
        //     let mut contract = CookieContract::new();

        //     // Set and get last stage
        //     assert_eq!(contract.set_last_stage(String::from("profile_stage")), Ok(()));
        //     assert_eq!(contract.get_last_stage(), Some(String::from("profile_stage")));
        // }
    }
}
