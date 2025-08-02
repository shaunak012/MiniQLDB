use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerEntry {
    pub id: String,
    pub data: Value,
    pub timestamp: i64,
    pub prevhash: String,
    pub hash: String,
}

impl LedgerEntry {
    pub fn new(id: String, data: Value, prevhash: String) -> LedgerEntry {
        let timestamp = Utc::now().timestamp();
        let mut entry = LedgerEntry {
            id: id,
            data: data,
            timestamp: timestamp,
            prevhash: prevhash,
            hash: String::new(),
        };
        entry.hash = entry.compute_hash();
        entry
    }
    pub fn compute_hash(&self) -> String {
        let json = serde_json::json!({
            "id":self.id,
            "data":self.data,
            "timestamp":self.timestamp,
            "prevhash":self.prevhash,
        });
        let serialized = serde_json::to_string(&json).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }
}

impl fmt::Display for LedgerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = chrono::DateTime::from_timestamp(self.timestamp, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S");
        write!(
            f,
            "[{}] ID: {} \nData: {}\nPrev Hash: {}\nHash: {}\n",
            time,
            self.id,
            serde_json::to_string_pretty(&self.data).unwrap(),
            self.prevhash,
            self.hash
        )
    }
}