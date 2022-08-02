use std::marker::{Send, Sync};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;

use crate::adapters::bidders::{Bidder, BidderKind};
use crate::adapters::test::{TestBidder, TestBidder2};
use crate::models::{
    AdCampaign, AdSource, AdxContext, BidderResponse, HttpCallInfo, HttpRequestData,
    HttpResponseData,
};

#[async_trait]
trait AdaptedBidder {
    async fn request_bid(
        &self,
        http_client: &Client,
        ctx: &AdxContext,
        ad_campaign: &AdCampaign,
        ad_source: &AdSource,
    ) -> Result<BidderResponse>;
}

struct BidderAdapter {
    bidder: BidderKind,
}

impl BidderAdapter {
    fn new(bidder: BidderKind) -> BidderAdapter {
        Self { bidder }
    }
}

#[async_trait]
impl AdaptedBidder for BidderAdapter {
    async fn request_bid(
        &self,
        http_client: &Client,
        ctx: &AdxContext,
        ad_campaign: &AdCampaign,
        ad_source: &AdSource,
    ) -> Result<BidderResponse> {
        let requestData = self
            .bidder
            .make_request(ctx, ad_campaign, ad_source)
            .await?;
        let call_info = self
            .do_request(http_client, ctx, ad_campaign, ad_source)
            .await?;
        let bid = self
            .bidder
            .make_bid(ctx, ad_campaign, &call_info.response)
            .await?;
        // todo!()
        Ok(BidderResponse {
            tag: bid.id,
            ad_source: None,
            ad_campaign: None,
            bid_response: None,
        })
    }
}

impl BidderAdapter {
    async fn do_request(
        &self,
        http_client: &Client,
        ctx: &AdxContext,
        ad_campaign: &AdCampaign,
        ad_source: &AdSource,
    ) -> Result<HttpCallInfo> {
        //todo!()
        Ok(HttpCallInfo {
            request: HttpRequestData {
                ad_source_code: "".to_string(),
                method: "".to_string(),
                uri: "".to_string(),
                body: None,
                headers: None,
            },
            response: HttpResponseData {
                status_code: 200,
                body: None,
                headers: None,
            },
            process_time: Default::default(),
        })
    }

    fn print_type_of(&self) {
        println!("print_type_of")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use futures::future::join_all;

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
        let media = Media {
            id: 10,
            name: "".to_string(),
        };
        let mut ctx = AdxContext {
            ip: "".to_string(),
            is_log: false,
            media: Arc::new(media),
        };
        let ad_campaign = &AdCampaign {};
        let ad_source = &AdSource {
            id: 1,
            bid_url: "ad1".to_string(),
            qps: 0,
        };

        let test = bidders::test::TestBidder {};
        let bidder_adapter1 = BidderAdapter::new(BidderKind::TestBidder(test));
        let req1 = bidder_adapter1.request_bid(&client, &ctx, ad_campaign, ad_source);

        let test = bidders::test::TestBidder2 {};
        let bidder_adapter2 = BidderAdapter::new(BidderKind::TestBidder2(test));
        let req2 = bidder_adapter2.request_bid(&client, &ctx, ad_campaign, ad_source);

        let req_list = vec![req1, req2];

        let results = join_all(req_list).await;

        for res in results {
            println!("{:?}", res);
        }
        println!("END")
    }
}
