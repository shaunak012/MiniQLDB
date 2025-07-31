use serde_json::json;

use crate::models::LedgerEntry;

mod cli;
mod ledger;
mod models;

fn main() {
    let entry = LedgerEntry::new(
        "user".to_string(),
        json!({"name":"Shaunak","coins":500}),
        "0".to_string(),
    );
    println!("{:#?}", entry);
}
