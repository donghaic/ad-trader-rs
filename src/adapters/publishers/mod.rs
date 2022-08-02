mod adx_api;

use anyhow::Result;
use async_trait::async_trait;
use openrtb::current::BidRequest;

use crate::models::*;

#[async_trait]
trait Publisher {
    async fn map_to_trader_bid_request(&self, context: &AdxContext) -> Result<BidRequest>;

    async fn map_to_publisher_bid_response(&self, context: &AdxContext) -> Result<HttpResponseData>;

    async fn decode_price(&self, context: &AdxContext) -> Result<f64>;
}