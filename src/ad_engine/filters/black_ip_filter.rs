use async_trait::async_trait;
use bloom::BloomFilter;

use crate::ad_engine::filters::AdFilter;
use crate::models::{AdCampaign, AdxContext};

struct BlackIpFilter {
    black_ip: BloomFilter,
}

impl BlackIpFilter {
    fn new(black_ip: BloomFilter) -> Self {
        BlackIpFilter { black_ip }
    }
}

#[async_trait]
impl AdFilter for BlackIpFilter {
    async fn is_match(&self, ctx: &AdxContext, ad_campaign: &AdCampaign) -> bool {
        if self.black_ip.contains(&ctx.ip) {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blackip() {
        let mut bloom = BloomFilter::with_rate(0.01, 100);
        bloom.insert(&"123".to_string());
        let exist = bloom.contains(&"123".to_string());
        println!("123 exist: {}", exist);
        assert_eq!(exist, true);

        let filter = BlackIpFilter::new(bloom);
        let res = filter.is_match(&AdxContext { ip: "123".to_owned() }, &AdCampaign {}).await;
        println!("res {}", res);
        assert_eq!(res, false);
    }
}