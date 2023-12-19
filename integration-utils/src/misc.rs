use std::{env, fs};

pub trait ToNear {
    fn to_near(&self) -> near_sdk::AccountId;
}

impl ToNear for near_workspaces::Account {
    fn to_near(&self) -> near_sdk::AccountId {
        near_sdk::AccountId::new_unchecked(self.id().to_string())
    }
}

pub fn load_wasm(wasm_path: &str) -> Vec<u8> {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let wasm_filepath = fs::canonicalize(current_dir.join(wasm_path))
        .expect("Failed to get wasm file path for: {wasm_path} current_dir: {current dir}");
    fs::read(wasm_filepath).expect("Failed to load wasm from: {wasm_filepath}")
}
