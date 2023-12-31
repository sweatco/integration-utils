use integration_utils::{contract_call::ContractCall, integration_contract::IntegrationContract};
use model::api::ContractApiIntegration;
use near_workspaces::Contract;

pub const MY_CONTRACT: &str = "my_contract";

pub struct MyContract<'a> {
    contract: &'a Contract,
}

impl ContractApiIntegration for MyContract<'_> {
    fn new(&self) -> ContractCall<()> {
        self.make_call("new")
    }

    fn test(&mut self) -> ContractCall<u32> {
        self.make_call("test")
    }

    fn data(&mut self) -> ContractCall<Vec<String>> {
        self.make_call("data")
    }

    fn log_and_panic(&mut self) -> ContractCall<()> {
        self.make_call("log_and_panic")
    }
}

impl<'a> IntegrationContract<'a> for MyContract<'a> {
    fn with_contract(contract: &'a Contract) -> Self {
        Self { contract }
    }

    fn contract(&self) -> &'a Contract {
        self.contract
    }
}
