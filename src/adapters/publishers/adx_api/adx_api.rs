use crate::adapters::publishers::Publisher;
use crate::models::{AdxContext, HttpResponseData};
use async_trait::async_trait;
use openrtb::current::BidRequest;

pub struct AdxApi {}

#[async_trait]
impl Publisher for AdxApi {
    async fn map_to_trader_bid_request(&self, context: &AdxContext) -> anyhow::Result<BidRequest> {
        todo!()
    }

    async fn map_to_publisher_bid_response(
        &self,
        context: &AdxContext,
    ) -> anyhow::Result<HttpResponseData> {
        todo!()
    }

    async fn decode_price(&self, context: &AdxContext) -> anyhow::Result<f64> {
        todo!()
    }
}
