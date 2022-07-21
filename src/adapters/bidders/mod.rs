use anyhow::Result;
use async_trait::async_trait;
use openrtb::current::BidResponse;

use crate::adapters::test::{TestBidder, TestBidder2};
use crate::models::*;

pub mod test;

#[async_trait]
pub(crate) trait Bidder: Send + Sync {
    async fn make_request(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<HttpRequestData>;

    async fn make_bid(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, res_data: &HttpResponseData) -> Result<BidResponse>;
}


pub(crate) enum BidderKind {
    TestBidder(TestBidder),
    TestBidder2(TestBidder2),
}

impl BidderKind {
    pub async fn make_request(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<HttpRequestData> {
        match self {
            Self::TestBidder(bidder) => bidder.make_request(ctx, ad_campaign, ad_source).await,
            Self::TestBidder2(bidder) => bidder.make_request(ctx, ad_campaign, ad_source).await,
        }
    }

    pub async fn make_bid(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, res_data: &HttpResponseData) -> Result<BidResponse> {
        match self {
            Self::TestBidder(bidder) => bidder.make_bid(ctx, ad_campaign, res_data).await,
            Self::TestBidder2(bidder) => bidder.make_bid(ctx, ad_campaign, res_data).await,
        }
    }
}