use crate::ledger::{append_entry, get_last_hash, read_all_block_entries, read_all_entries};
use crate::merkle::compute_merkle_root;
use crate::models::LedgerEntry;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "MiniQLDB",
    version = "0.1",
    about = "Lightweight, imutable ledger"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { id: String, json_data: String },
    Get { id: String },
    History { id: String },
    VerifyChain,
    BuildBlock,
    ListBlocks,
    VerifyBlocks,
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { id, json_data } => {
            let data: serde_json::Value =
                serde_json::from_str(&json_data).expect("Invalid JSON Format");
            let prev_hash = get_last_hash().unwrap();
            let entry = LedgerEntry::new(id.clone(), data, prev_hash);
            append_entry(&entry).expect("Failed to write ledger");
            println!("Added ID: {}", id);
        }
        Commands::Get { id } => {
            let entries = read_all_entries().unwrap();
            let latest = entries.iter().rev().find(|e| e.id == id);
            match latest {
                Some(e) => println!("{}", e),
                None => println!("No entry found for ID: {}", id),
            }
        }
        Commands::History { id } => {
            let entries = read_all_entries().unwrap();
            let mut history: Vec<_> = entries.into_iter().filter(|e| e.id == id).collect();
            if history.is_empty() {
                println!("No histroy found for ID: {}", id);
            } else {
                history.sort_by_key(|e| e.timestamp);
                for entry in history {
                    println!("{}", entry);
                }
            }
        }
        Commands::VerifyChain => {
            let entries = read_all_entries().unwrap();
            if entries.len() < 2 {
                println!("Not enough entries");
            } else {
                for i in 1..entries.len() {
                    let first_hash = &entries[i - 1];
                    let second_hash = &entries[i];
                    if first_hash.hash != second_hash.prevhash {
                        println!(
                            "Tampering Detected at entries:\n\n{}\n{}",
                            entries[i-1], entries[i]
                        );
                        return;
                    }
                }
            }
            println!("Chain is intact")
        }
        Commands::BuildBlock => {
            let entries = read_all_entries().unwrap();
            if entries.len() < 5 {
                println!("Atleast 5 entries are required");
                return;
            }
            let block_entries = entries[..5].to_vec();
            let block = crate::merkle::MerkleBlock::new(block_entries);
            crate::ledger::write_merkle_block(&block).expect("Failed to write Merkle block");
            println!("{} {}","Block created with Merkle Root:".green(), block.merkle_root);
        }
        Commands::ListBlocks => {
            let blocks = read_all_block_entries().unwrap();
            if blocks.is_empty() {
                println!("No Blocks Found");
            } else {
                for block in blocks {
                    println!("{}", block);
                }
            }
        }
        Commands::VerifyBlocks =>{
            let blocks = read_all_block_entries().unwrap();
            for i in 0..blocks.len(){
                let verification_hash= compute_merkle_root(&blocks[i].entries);
                if &verification_hash != &blocks[i].merkle_root{
                    println!("{}{}","Tampering Detected in Merkle root:\n".red(),blocks[i]);
                    return;
                }
            }
            println!("{}","Verification of Merkleblocks is OK".green())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_integrity (){
        let entries=read_all_entries().unwrap();
        for i in 1..entries.len(){
            assert_eq!(entries[i-1].hash,entries[i].prevhash);
        }
    }

    #[test]
    fn test_block_integrity(){
        let blocks = read_all_block_entries().unwrap();
        for block in blocks{
            let verification_hash = compute_merkle_root(&block.entries);
            assert_eq!(verification_hash,block.merkle_root);
        }
    }
}

