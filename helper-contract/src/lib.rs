pub mod api;
pub mod interface;

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, PanicOnDefault, Timestamp,
};

use crate::api::HelperApi;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
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

    fn block_timestamp_ms(&self) -> Timestamp {
        env::block_timestamp_ms()
    }
}
