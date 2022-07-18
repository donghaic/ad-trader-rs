use crate::models::{AdxContext, BidderResponse};
use async_trait::async_trait;
mod creative;


#[async_trait]
trait AdValidator {
    async fn is_ok(&self, ctx: &AdxContext, response: &BidderResponse) -> bool;
}