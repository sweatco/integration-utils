pub mod api;

use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    env, near_bindgen, PanicOnDefault, Timestamp,
};

use crate::api::HelperApi;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    /// Some data
    data: String,
}

#[near_bindgen]
impl HelperApi for Contract {
    #[init]
    #[private]
    fn new() -> Self {
        Self {
            data: "hello".to_string(),
        }
    }

    fn block_timestamp_ms(&self, some_value: String) -> Timestamp {
        dbg!(some_value);
        env::block_timestamp_ms()
    }
}
