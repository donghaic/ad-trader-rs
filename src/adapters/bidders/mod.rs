use anyhow::Result;
use async_trait::async_trait;
use openrtb::current::BidResponse;

use crate::models::*;

#[async_trait]
pub trait Bidder {
    async fn make_request(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<HttpRequestData>;

    async fn make_bid(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, res_data: &HttpResponseData) -> Result<BidResponse>;
}