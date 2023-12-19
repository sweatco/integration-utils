use integration_trait::make_integration_version;
use near_sdk::Timestamp;

#[make_integration_version]
pub trait HelperApi {
    fn new() -> Self;
    fn block_timestamp_ms(&self) -> Timestamp;
}
