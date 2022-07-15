use axum::body::Body;
use axum::http::header::HeaderMap;

use bytes::Bytes;

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
    pub body: Bytes,
    pub headers: HeaderMap,
}
