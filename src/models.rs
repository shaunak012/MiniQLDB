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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let time = match chrono::DateTime::from_timestamp(self.timestamp, 0){
            Some(time) => time.format("%d-%m-%Y %H:%M:%S").to_string(),
            None => return Err(fmt::Error),
        };

        let pretty_data=serde_json::to_string_pretty(&self.data).map_err(|_| fmt::Error)?;
        
        write!(
            f,
            "[{}] \nID: {} \nData: {}\nPrev Hash: {}\nHash: {}\n",
            time,
            self.id,
            pretty_data,
            self.prevhash,
            self.hash
        )
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use serde_json::json;

    #[test]
    fn test_hash_consistency(){
        let entry1= LedgerEntry::new("test".to_string(),json!({"a":1}),"prevhash".to_string());
        let entry2= LedgerEntry::new("test".to_string(),json!({"a":1}),"prevhash".to_string());
        
        assert_eq!(entry1.hash, entry2.hash);
    }

    #[test]
    fn test_hash_changes_with_data(){
        let entry1= LedgerEntry::new("test".to_string(),json!({"a":1}),"prevhash".to_string());
        let entry2= LedgerEntry::new("test".to_string(),json!({"a":2}),"prevhash".to_string());
        
        assert_ne!(entry1.hash, entry2.hash);
    }
}