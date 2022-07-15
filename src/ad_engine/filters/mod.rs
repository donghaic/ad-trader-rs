use async_trait::async_trait;

use crate::models::{AdCampaign, AdxContext};

mod black_ip_filter;

#[async_trait]
trait AdFilter {
    async fn is_match(&self, ctx: &AdxContext, ad_campaign: &AdCampaign) -> bool;
}