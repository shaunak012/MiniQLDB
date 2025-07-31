mod cli;
mod ledger;
mod models;

use serde_json::json;
use models::LedgerEntry;
use crate::ledger::{append_entry, read_all_entries, get_last_hash};

fn main() {
    let prev_hash = get_last_hash().unwrap();
    let entry = LedgerEntry::new(
        "user".to_string(),
        json!({"name":"Shaunak2","coins":500}),
        prev_hash,
    );
    println!("{:#?}", entry);
    
    append_entry(&entry).unwrap();
    println!("All ledger Entries:");
    let all_entries = read_all_entries().unwrap();
    for entry in all_entries{
        println!("{:#?}",entry);
    }
}
