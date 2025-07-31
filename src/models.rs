use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize,Deserialize,)]
pub struct LedgerEntry {
    pub id: String,
    pub data: Value,
    pub timestamp: DateTime<Utc>,
    pub prevhash: String,
    pub hash: String,
}

impl LedgerEntry {
    pub fn new(id: String, data: Value, prevhash: String) -> Self {
        let timestamp = Utc::now();
        let mut entry = LedgerEntry{
            id:id,
            data:data,
            timestamp:timestamp,
            prevhash:prevhash,
            hash:String::new(),
        };
        entry.hash = entry.compute_hash();
    }
    pub fn compute_hash(&self)->String{
        let json=serde_json::json!({
            "id":self.id,
            "data":self.data,
            "timestamp":self.timestamp,
            "prevhash":self.prevhash,
        })
    }
}
