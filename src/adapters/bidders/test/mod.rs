use anyhow::anyhow;
use async_trait::async_trait;
use openrtb::current::BidResponse;

use crate::adapters::bidders::Bidder;
use crate::models::{AdCampaign, AdSource, AdxContext, HttpRequestData, HttpResponseData};

pub struct TestBidder;


#[async_trait]
impl Bidder for TestBidder {
    async fn make_request(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> anyhow::Result<HttpRequestData> {
        return Err(anyhow!("TDO"));
    }

    async fn make_bid(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, res_data: &HttpResponseData) -> anyhow::Result<BidResponse> {
        return Err(anyhow!("TDO"));
    }
}