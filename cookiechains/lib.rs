/*Source: https://github.com/Cardinal-Cryptography/bulletin-board-example/blob/main/contracts/bulletin_board/lib.rs*/
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
    }

    // impl From<ink::env::Error> for Error {
    //     fn from(e: ink::env::Error) -> Self {
    //         Error::InkEnvError(format!("{:?}", e))
    //     }
    // }

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
    }

    type Event = <CookieContract as ink::reflect::ContractEventBase>::Type;

    #[ink(event)]
    pub struct CookieRegistered {
        cookie: String,
        owner: AccountId,
        block: BlockNumber,
    }

    #[ink(event)]
    pub struct CookieUpdated {
        cookie: String,
        owner: AccountId,
        block: BlockNumber,
    }

    #[ink(event)]
    pub struct CookieDeleted {
        cookie: String,
        owner: AccountId,
        block: BlockNumber,
    }

    #[ink(storage)]
    pub struct CookieContract {
        owner: AccountId,
        cookie_count: u32,
        // Mapping from cookie string to cookieEntry
        cookies: Mapping<String, CookieEntry>,
        // List of all registered cookies
        cookie_list: Vec<String>,
        // Mapping from account to owned cookies
        owned_cookies: Mapping<AccountId, Vec<String>>,
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
            }
        }

        // Register a new cookie
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
            });

            Ok(())
        }

        /// Update an existing cookie
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
            let entry = self.cookies.get(&cookie).ok_or(Error::CookieNotFound)?;
            if entry.owner != caller {
                return Err(Error::NotAuthorized);
            }

            let cookie_entry = CookieEntry {
                profile,
                cookie: cookie.clone(),
                expiration_date,
                name,
                secure,
                path,
                value,
                created_at: entry.created_at,
                owner: caller,
            };

            self.cookies.insert(&cookie, &cookie_entry);

            self.env().emit_event(CookieUpdated {
                cookie,
                owner: caller,
                block: current_block,
            });

            Ok(())
        }

        // Delete a cookie
        #[ink(message)]
        pub fn delete_cookie(&mut self, cookie: String) -> Result<(), Error> {
            let caller = self.env().caller();
            let current_block = self.env().block_number();

            let entry = self.cookies.get(&cookie).ok_or(Error::CookieNotFound)?;
            if entry.owner != caller && caller != self.owner {
                return Err(Error::NotAuthorized);
            }

            // Update storage
            self.cookies.remove(&cookie);
            self.cookie_list.retain(|x| x != &cookie);
            self.cookie_count = self.cookie_count.wrapping_sub(1);

            // Update owned cookies
            if let Some(mut owned) = self.owned_cookies.get(entry.owner) {
                owned.retain(|x| x != &cookie);
                self.owned_cookies.insert(entry.owner, &owned);
            }

            self.env().emit_event(CookieDeleted {
                cookie,
                owner: caller,
                block: current_block,
            });

            Ok(())
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
//  Basic add cookie test

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
// Duplicate maybe allowed? 
        #[ink::test]
        fn duplicate_cookie_fails() {
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
                contract.register_cookie(
                    String::from("profile2"),
                    String::from("cookie1"),
                    String::from("2024-12-31"),
                    String::from("name2"),
                    String::from("secure2"),
                    String::from("/path2"),
                    String::from("value2"),
                ),
                Err(Error::CookieAlreadyExists)
            );
        }
//Update data fields
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
    }
}