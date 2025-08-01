// for ledger operation
use crate::models::LedgerEntry;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const LEDGER_FILE: &str = "ledger.jsonl";

pub fn append_entry(entry: &LedgerEntry) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LEDGER_FILE)?;
    let json_line = serde_json::to_string(entry)?;
    writeln!(file, "{}", json_line)?;
    Ok(())
}

pub fn read_all_entries() -> std::io::Result<Vec<LedgerEntry>> {
    let file = match File::open(LEDGER_FILE) {
        Ok(f) => f,
        Err(_) => return Ok(vec![]),
    };
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let entry: LedgerEntry = serde_json::from_str(&line)?;
        entries.push(entry);
    }
    Ok(entries)
}

pub fn get_last_hash() -> std::io::Result<String> {
    let entries = read_all_entries()?;
    if let Some(last) = entries.last() {
        Ok(last.hash.clone())
    } else {
        Ok("0".to_string())
    }
}
