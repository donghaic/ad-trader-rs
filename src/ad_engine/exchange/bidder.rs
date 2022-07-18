use std::marker::{Send, Sync};

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

use crate::adapters::bidders::Bidder;
use crate::models::{AdCampaign, AdSource, AdxContext, BidderResponse, HttpCallInfo, HttpRequestData};

#[async_trait]
trait AdaptedBidder {
    async fn request_bid(&self, http_client: &Client, ctx: &mut AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<BidderResponse>;
}

struct BidderAdapter<T: Bidder + Send + Sync> {
    bidder: T,
}

impl<T: Bidder + Send + Sync> BidderAdapter<T> {
    fn new(bidder: T) -> BidderAdapter<T> {
        Self {
            bidder
        }
    }
}


#[async_trait]
impl<T: Bidder + Send + Sync> AdaptedBidder for BidderAdapter<T> {
    async fn request_bid(&self, http_client: &Client, ctx: &mut AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<BidderResponse> {
        let requestData = self.bidder.make_request(ctx, ad_campaign, ad_source).await?;
        // let call_info = self.do_request(http_client, ctx, ad_campaign, ad_source).await?;
        // let bid = self.bidder.make_bid(ctx, ad_campaign, &call_info.response).await?;
        todo!()
    }
}

impl<T: Bidder + Send + Sync> BidderAdapter<T> {
    async fn do_request(&self, http_client: &Client, ctx: &mut AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<HttpCallInfo> {
        todo!()
    }

    fn print_type_of(&self) {
        println!("print_type_of")
    }
}


#[cfg(test)]
mod tests {
    use crate::adapters::bidders;

    use super::*;

    #[tokio::test]
    async fn test_client() {
        let _client = reqwest::Client::new();
    }


    #[tokio::test]
    async fn test_bidder() {
        let test = bidders::test::TestBidder {};
        let bidder_adapter = BidderAdapter::new(test);
        bidder_adapter.print_type_of();
    }
}