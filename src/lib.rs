// #![cfg_attr(not(feature = "std"), no_std)]

// pub use ink::prelude::vec::Vec;
// use ink::storage::Mapping;
// use ink_lang as ink;

// #[ink::contract]
// mod cookiechain {

//     #[ink(storage)]
//     pub struct Cookiechain {
//         cookies_count: u32,
//         cookies: Mapping<u32, Cookie>,
//     }

//     #[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
//     #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
//     pub struct Cookie {
//         profile: Vec<u8>,
//         domain: Vec<u8>,
//         expiration_date: Vec<u8>,
//         name: Vec<u8>,
//         secure: Vec<u8>,
//         path: Vec<u8>,
//         value: Vec<u8>,
//     }

//     impl Cookiechain {
//         #[ink(constructor)]
//         pub fn new() -> Self {
//             Self {
//                 cookies_count: 0,
//                 cookies: Mapping::new(),
//             }
//         }

//         #[ink(message)]
//         pub fn create_cookie(
//             &mut self,
//             profile: Vec<u8>,
//             domain: Vec<u8>,
//             expiration_date: Vec<u8>,
//             name: Vec<u8>,
//             secure: Vec<u8>,
//             path: Vec<u8>,
//             value: Vec<u8>,
//         ) {
//             self.cookies_count += 1;
//             let cookie = Cookie {
//                 profile,
//                 domain,
//                 expiration_date,
//                 name,
//                 secure,
//                 path,
//                 value,
//             };
//             self.cookies.insert(self.cookies_count, &cookie);
//         }

//         #[ink(message)]
//         pub fn update_cookie(
//             &mut self,
//             index: u32,
//             profile: Vec<u8>,
//             domain: Vec<u8>,
//             expiration_date: Vec<u8>,
//             name: Vec<u8>,
//             secure: Vec<u8>,
//             path: Vec<u8>,
//             value: Vec<u8>,
//         ) {
//             let cookie = Cookie {
//                 profile,
//                 domain,
//                 expiration_date,
//                 name,
//                 secure,
//                 path,
//                 value,
//             };
//             self.cookies.insert(index, &cookie);
//         }

//         #[ink(message)]
//         pub fn get_cookie(&self, index: u32) -> Option<Cookie> {
//             self.cookies.get(index)
//         }

//         #[ink(message)]
//         pub fn get_cookies_count(&self) -> u32 {
//             self.cookies_count
//         }
//     }
// }
