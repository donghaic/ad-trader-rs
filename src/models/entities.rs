#[derive(Debug)]
pub struct AdCampaign {
    // TODO
}

#[derive(Debug)]
pub struct AdSource {
    pub id: u32,
    pub bid_url: String,
    pub qps: u32,
}

#[derive(Debug)]
pub struct Media {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct Adslot {
    pub id: u32,
    pub name: String,
}