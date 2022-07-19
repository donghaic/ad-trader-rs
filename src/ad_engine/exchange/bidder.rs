use std::marker::{Send, Sync};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;

use crate::adapters::bidders::Bidder;
use crate::models::{AdCampaign, AdSource, AdxContext, BidderResponse, HttpCallInfo, HttpRequestData};

#[async_trait]
trait AdaptedBidder {
    async fn request_bid(&self, http_client: &Client, ctx: &mut AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<BidderResponse>;
}

struct BidderAdapter<T: Bidder> {
    bidder: T,
}

impl<T: Bidder> BidderAdapter<T> {
    fn new(bidder: T) -> BidderAdapter<T> {
        Self {
            bidder
        }
    }
}


#[async_trait]
impl<T: Bidder> AdaptedBidder for BidderAdapter<T> {
    async fn request_bid(&self, http_client: &Client, ctx: &mut AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<BidderResponse> {
        let requestData = self.bidder.make_request(ctx, ad_campaign, ad_source).await?;
        // let call_info = self.do_request(http_client, ctx, ad_campaign, ad_source).await?;
        //let bid = self.bidder.make_bid(ctx, ad_campaign, &call_info.response).await?;
        // todo!()
        Ok(BidderResponse {
            ad_source: None,
            ad_campaign: None,
            bid_response: None,
        })
    }
}

impl<T: Bidder> BidderAdapter<T> {
    async fn do_request(&self, http_client: &Client, ctx: &mut AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<HttpCallInfo> {
        todo!()
    }

    fn print_type_of(&self) {
        println!("print_type_of")
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::adapters::bidders;
    use crate::models::*;

    use super::*;

    #[tokio::test]
    async fn test_client() {
        let _client = reqwest::Client::new();
    }


    #[tokio::test]
    async fn test_bidder() {
        let client = reqwest::Client::new();
        let mut ctx = AdxContext { ip: "".to_string() };
        let ad_campaign = &AdCampaign {};
        let ad_source = &AdSource {
            id: 1,
            bid_url: "ad1".to_string(),
            qps: 0,
        };

        let test = bidders::test::TestBidder {};
        let bidder_adapter1 = BidderAdapter::new(test);
        bidder_adapter1.request_bid(&client, &mut ctx, ad_campaign, ad_source).await;

        let test = bidders::test::TestBidder2 {};
        let bidder_adapter2 = BidderAdapter::new(test);
        bidder_adapter2.request_bid(&client, &mut ctx, ad_campaign, ad_source).await;

    }
}