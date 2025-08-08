use crate::models::LedgerEntry;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

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

impl fmt::Display for MerkleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let time = match chrono::DateTime::from_timestamp(self.timestamp, 0) {
            Some(time) => time.format("%d-%m-%Y %H:%M:%S").to_string(),
            None => return Err(fmt::Error),
        };

        let pretty_data = serde_json::to_string_pretty(&self.entries).map_err(|_| fmt::Error)?;

        write!(
            f,
            "[{}] \nMerkle Root: {}\nEntries: {}\n",
            time, self.merkle_root, pretty_data
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_hash: String,
    pub path: Vec<(String, bool)>,
}

pub fn generate_merkle_proof(entries: &[LedgerEntry], target_hash: &str) -> Option<MerkleProof> {
    let hashes: Vec<String> = entries.iter().map(|e| e.hash.clone()).collect();
    let mut path = Vec::new();
    let mut index = hashes.iter().position(|i| i == target_hash)?;
    let mut level_hashes = hashes.clone();

    while level_hashes.len() > 1 {
        if level_hashes.len() % 2 != 0 {
            level_hashes.push(level_hashes.last().unwrap().clone());
        }
        let mut next_level = Vec::new();
        for i in (0..level_hashes.len()).step_by(2) {
            let left = &level_hashes[i];
            let right = &level_hashes[i + 1];
            let combined = format!("{}{}", left, right);
            let mut hasher = Sha256::new();
            hasher.update(combined);
            let parent = format!("{:x}", hasher.finalize());

            if i == index || i + 1 == index {
                let is_left = i != index;
                let sibling_hash = if i == index {
                    right.clone()
                } else {
                    left.clone()
                };
                path.push((sibling_hash, is_left));
                index = next_level.len();
            }

            next_level.push(parent);
        }
        level_hashes = next_level
    }
    Some(MerkleProof {
        leaf_hash: target_hash.to_string(),
        path,
    })
}

pub fn verify_merkle_proof(proof: &MerkleProof, root: &str) -> bool {
    let mut hash = proof.leaf_hash.clone();
    for (sibling, is_left) in &proof.path {
        let combined = if *is_left {
            format!("{}{}", sibling, hash)
        } else {
            format!("{}{}", hash, sibling)
        };
        let mut hasher = Sha256::new();
        hasher.update(combined);
        hash = format!("{:x}", hasher.finalize());
    }
    hash == root
}
