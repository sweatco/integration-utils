#![cfg(not(target_arch = "wasm32"))]

use integration_utils::{contract_call::ContractCall, integration_contract::IntegrationContract, misc::ToNear};
use near_sdk::{AccountId, Timestamp};
use near_workspaces::Contract;

use crate::api::HelperApiIntegration;

pub const HELPER_CONTRACT: &str = "helper_contract";

pub struct HelperContract<'a> {
    contract: &'a Contract,
}

impl<'a> HelperContract<'a> {
    pub fn new(contract: &'a Contract) -> Self {
        Self { contract }
    }
}

impl HelperApiIntegration for HelperContract<'_> {
    fn new(&self) -> ContractCall<()> {
        self.make_call("new")
    }

    fn block_timestamp_ms(&self) -> ContractCall<Timestamp> {
        self.make_call("block_timestamp_ms")
    }
}

impl<'a> IntegrationContract<'a> for HelperContract<'a> {
    fn with_contract(contract: &'a Contract) -> Self {
        Self { contract }
    }

    fn contract(&self) -> &'a Contract {
        &self.contract
    }
}

pub trait GetContractAccount {
    fn contract_account(&self) -> AccountId;
}

impl<'a, T: IntegrationContract<'a>> GetContractAccount for T {
    fn contract_account(&self) -> AccountId {
        self.contract().as_account().to_near()
    }
}
