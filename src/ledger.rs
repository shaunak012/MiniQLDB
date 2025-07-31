// for ledger operation

use std::fs::{OpenOptions, File};
use std::io::{Write};
use crate::models::LedgerEntry;

const LEDGER_FILE: &str= "ledger.jsonl";

pub fn append_entry(entry: &LedgerEntry) -> std::io::Result<()>{
    let mut file = OpenOptions::new().create(true).append(true).open(LEDGER_FILE)?;
    let json_line=serde_json::to_string(entry)?;
    writeln!(file,"{}",json_line);
    Ok(())
}

pub read_all_entries() -> std::io::Result<Vec<LedgerEntry>>{
    //Todo
}


pub fn get_last_hash() -> std::io::Result<String>{
    let entries = read_all_entries();
    if let Some(last) = entries.last(){
        Ok(last.hash.clone())
    } else {
        Ok("0".to_string())
    }
}