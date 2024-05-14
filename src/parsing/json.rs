use crate::types::album::Album;

pub struct JsonParser {}

impl JsonParser {
    pub fn get_album_from_json(&self, json: String) -> Album {
        return serde_json::from_str(&json).unwrap();
    }

    pub fn get_json_from_album(&self, album: Album) -> String {
        return serde_json::to_string(&album).unwrap();
    }
}
