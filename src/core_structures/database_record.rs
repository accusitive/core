use std::collections::HashMap;

pub struct DatabaseRecord {
    pub name: String,
    pub tables: HashMap<String, (u32, u64)>
}

impl DatabaseRecord {
    // pub fn new()
}