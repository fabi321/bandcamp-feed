use serde_derive::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct GenreRecord {
    pub genre: Vec<String>,
    pub location: Option<u32>,
    pub category: Option<u32>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum RecordType {
    /// For future proofing, in case I want to add user feeds
    #[allow(dead_code)]
    UserRecord(String),
    GenreRecord(GenreRecord),
}

#[derive(Clone, Deserialize)]
pub struct QueryResult {
    pub title: String,
    pub item_url: String,
    pub item_price: f64,
    pub item_currency: String,
    pub item_image_id: i64,
    pub result_type: String,
    pub band_name: String,
    pub band_url: String,
    pub release_date: DateTime<Utc>,
    pub band_location: Option<String>,
    pub track_count: Option<i64>,
    pub item_duration: Option<f64>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Errors {
    NotFound,
    InternalError,
}
