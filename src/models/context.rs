use core::time;

use axum::body::Body;
use axum::http::header::HeaderMap;
use bytes::Bytes;
use openrtb::current::BidResponse;

use crate::models::{AdCampaign, AdSource};

pub struct AdxContext {
    // todo
    pub ip: String,
}


pub struct HttpResponseData {
    pub status_code: i16,
    pub body: Bytes,
    pub headers: HeaderMap,
}

pub struct HttpRequestData {
    pub ad_source_code: String,
    pub method: String,
    pub uri: String,
    pub body: Option<Bytes>,
    pub headers: Option<HeaderMap>,
}

pub struct BidderResponse {
    pub ad_source: Option<AdSource>,
    pub ad_campaign: Option<AdCampaign>,
    pub bid_response: Option<BidResponse>,
}


pub struct HttpCallInfo {
    pub request: HttpRequestData,
    pub response: HttpResponseData,
    pub process_time: time::Duration,
}