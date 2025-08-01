mod cli;
mod ledger;
mod models;

use serde_json::json;
use models::LedgerEntry;
use crate::ledger::{append_entry, read_all_entries, get_last_hash};

fn main() {
    cli::run();
}
