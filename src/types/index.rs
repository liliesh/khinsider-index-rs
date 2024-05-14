pub struct SearchIndex {
    pub index_version: String,
    pub entries: Vec<IndexEntry>,
}

pub struct IndexEntry {
    pub display_name: String,
    pub slug: String,
    pub path: String,
}
