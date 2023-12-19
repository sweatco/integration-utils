#![cfg(test)]

use async_trait::async_trait;
use helper_contract::{
    api::HelperApiIntegration,
    interface::{HelperContract, HELPER_CONTRACT},
};
use integration_utils::integration_contract::IntegrationContract;
use model::api::ContractApiIntegration;
use near_workspaces::Account;

use crate::contract_interface::{MyContract, MY_CONTRACT};

pub type Context = integration_utils::context::Context<near_workspaces::network::Sandbox>;

#[async_trait]
pub trait IntegrationContext {
    async fn manager(&mut self) -> anyhow::Result<Account>;
    async fn alice(&mut self) -> anyhow::Result<Account>;
    async fn fee(&mut self) -> anyhow::Result<Account>;
    fn my_contract(&self) -> MyContract;
    fn helper(&self) -> HelperContract;
}

#[async_trait]
impl IntegrationContext for Context {
    async fn manager(&mut self) -> anyhow::Result<Account> {
        self.account("manager").await
    }

    async fn alice(&mut self) -> anyhow::Result<Account> {
        self.account("alice").await
    }

    async fn fee(&mut self) -> anyhow::Result<Account> {
        self.account("fee").await
    }

    fn my_contract(&self) -> MyContract {
        MyContract::with_contract(&self.contracts[MY_CONTRACT])
    }

    fn helper(&self) -> HelperContract {
        HelperContract::with_contract(&self.contracts[HELPER_CONTRACT])
    }
}

pub(crate) async fn prepare_contract() -> anyhow::Result<Context> {
    let context = Context::new(&[MY_CONTRACT, HELPER_CONTRACT], "build-integration".into()).await?;

    context.my_contract().new().call().await?;
    context.helper().new().call().await?;

    Ok(context)
}
