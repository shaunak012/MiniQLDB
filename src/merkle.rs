use crate::models::LedgerEntry;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleBlock {
    pub entries: Vec<LedgerEntry>,
    pub merkle_root: String,
    pub timestamp: i64,
}

impl MerkleBlock {
    pub fn new(entries: Vec<LedgerEntry>) -> Self {
        let merkle_root = compute_merkle_root(&entries);
        let timestamp = chrono::Utc::now().timestamp();
        MerkleBlock {
            entries,
            merkle_root,
            timestamp,
        }
    }
}

pub fn compute_merkle_root(entries: &[LedgerEntry]) -> String {
    let mut hashes: Vec<String> = entries.iter().map(|e| e.hash.clone()).collect();
    if hashes.is_empty() {
        return "Empty".to_string();
    }
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }

        let mut next_level = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            let combined = format!("{}{}", hashes[i], hashes[i + 1]);
            let mut hasher = Sha256::new();
            hasher.update(combined);
            next_level.push(format!("{:x}", hasher.finalize()));
        }
        hashes = next_level;
    }
    return hashes[0].clone();
}
