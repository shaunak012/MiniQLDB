use serde_json::Value;

pub struct LedgerEntry{
    pub id: String,
    pub data: Value,
    pub timestamp: i64,
    pub prevhash: String,
    pub hash: String
}

