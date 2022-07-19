use anyhow::anyhow;
use async_trait::async_trait;
use openrtb::current::BidResponse;

use crate::adapters::bidders::Bidder;
use crate::models::{AdCampaign, AdSource, AdxContext, HttpRequestData, HttpResponseData};

pub struct TestBidder;


#[async_trait]
impl Bidder for TestBidder {
    async fn make_request(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> anyhow::Result<HttpRequestData> {
        println!("TestBidder make_request");

        Ok(HttpRequestData {
            ad_source_code: "1".to_string(),
            method: "POST".to_string(),
            uri: "1".to_string(),
            body: None,
            headers: None,
        })
    }

    async fn make_bid(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, res_data: &HttpResponseData) -> anyhow::Result<BidResponse> {
        println!("TestBidder make_bid");
        return Err(anyhow!("TDO"));
    }
}

pub struct TestBidder2;


#[async_trait]
impl Bidder for TestBidder2 {
    async fn make_request(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> anyhow::Result<HttpRequestData> {
        println!("TestBidder2 make_request");

        Ok(HttpRequestData {
            ad_source_code: "2".to_string(),
            method: "POST".to_string(),
            uri: "2".to_string(),
            body: None,
            headers: None,
        })
    }

    async fn make_bid(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, res_data: &HttpResponseData) -> anyhow::Result<BidResponse> {
        println!("TestBidder2 make_bid");

        return Err(anyhow!("TDO"));
    }
}