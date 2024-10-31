#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)] // Ensure this line is present

use ink::prelude::string::String; // Import String from ink's prelude
use ink::storage::Mapping; // Import Mapping from ink's storage

#[ink::contract]
pub mod cookiechain {
    #[ink(storage)]
    pub struct Cookiechain {
        cookies_count: u32,
        cookies: Mapping<u32, Cookie>,
    }

    #[derive(Default, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Cookie {
        profile: String,
        domain: String,
        expiration_date: String,
        name: String,
        secure: String,
        path: String,
        value: String,
    }

    impl Cookiechain {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                cookies_count: 0,
                cookies: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn create_cookie(
            &mut self,
            profile: String,
            domain: String,
            expiration_date: String,
            name: String,
            secure: String,
            path: String,
            value: String,
        ) {
            let index = self.cookies_count; // Use current count as index
            let cookie = Cookie {
                profile,
                domain,
                expiration_date,
                name,
                secure,
                path,
                value,
            };
            self.cookies.insert(index, &cookie);
            self.cookies_count += 1; // Update the cookie count after inserting
        }

        #[ink(message)]
        pub fn update_cookie(
            &mut self,
            index: u32,
            profile: String,
            domain: String,
            expiration_date: String,
            name: String,
            secure: String,
            path: String,
            value: String,
        ) {
            // Check if the cookie exists before updating
            if self.cookies.contains_key(&index) {
                let cookie = Cookie {
                    profile,
                    domain,
                    expiration_date,
                    name,
                    secure,
                    path,
                    value,
                };
                self.cookies.insert(index, &cookie);
            } else {
                // Handle the error (e.g., log, revert, etc.)
                panic!("Cookie index does not exist.");
            }
        }
    }
}
