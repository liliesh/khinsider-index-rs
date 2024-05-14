use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Album {
    pub title: String,
    pub titles_alternative: Vec<String>,
    pub crawled_at: NaiveDateTime,
    pub slug: String,
    pub url: String,
    pub platforms: HashMap<String, String>,
    pub images: Vec<String>,
    pub year: u8,
    pub date_added: NaiveDateTime,
    pub developers: HashMap<String, String>,
    pub publishers: HashMap<String, String>,
    pub genres: HashMap<String, String>,
    pub tracks: Vec<Track>,
    pub total: AlbumTotal,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub disc_number: u8,
    pub filesize_flac_bytes: u64,
    pub filesize_mp3_bytes: u64,
    pub track_number: u16,
    pub title: String,
    pub runtime: u32,
    pub source_mp3: String,
    pub source_flac: String,
    pub track_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct AlbumTotal {
    pub runtime: u64,
    pub filesize_mp3_bytes: u128,
    pub filesize_flac_bytes: u128,
    pub tracks: u32,
}
