#![cfg(test)]

use helper_model::api::{HelperApiIntegration, HelperContract};
use my_model::api::{ContractApiIntegration, MyContract};
use near_workspaces::Account;

pub type Context = nitka::context::Context<near_workspaces::network::Sandbox>;

pub const HELPER_CONTRACT: &str = "helper_contract";
pub const MY_CONTRACT: &str = "my_contract";

pub trait IntegrationContext {
    async fn manager(&mut self) -> anyhow::Result<Account>;
    async fn alice(&mut self) -> anyhow::Result<Account>;
    async fn fee(&mut self) -> anyhow::Result<Account>;
    fn my_contract(&self) -> MyContract;
    fn helper(&self) -> HelperContract;
}

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
        MyContract {
            contract: &self.contracts[MY_CONTRACT],
        }
    }

    fn helper(&self) -> HelperContract {
        HelperContract {
            contract: &self.contracts[HELPER_CONTRACT],
        }
    }
}

pub(crate) async fn prepare_contract() -> anyhow::Result<Context> {
    let context = Context::new(&[MY_CONTRACT, HELPER_CONTRACT], true, "build-integration".into()).await?;

    context.my_contract().new().await?;
    context.helper().new().await?;

    Ok(context)
}
