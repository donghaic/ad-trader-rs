use core::time;
use std::sync::Arc;

use axum::body::Body;
use axum::http::header::HeaderMap;
use bytes::Bytes;
use openrtb::current::BidResponse;

use crate::models::{AdCampaign, AdSource, Media};

pub struct AdxContext {
    // todo
    pub ip: String,
    pub is_log: bool,
    pub media: Arc<Media>,
}


pub struct HttpResponseData {
    pub status_code: i16,
    pub body: Option<Bytes>,
    pub headers: Option<HeaderMap>,
}

pub struct HttpRequestData {
    pub ad_source_code: String,
    pub method: String,
    pub uri: String,
    pub body: Option<Bytes>,
    pub headers: Option<HeaderMap>,
}

#[derive(Debug)]
pub struct BidderResponse {
    pub tag: String,
    pub ad_source: Option<AdSource>,
    pub ad_campaign: Option<AdCampaign>,
    pub bid_response: Option<BidResponse>,
}


pub struct HttpCallInfo {
    pub request: HttpRequestData,
    pub response: HttpResponseData,
    pub process_time: time::Duration,
}