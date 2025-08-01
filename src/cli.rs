use clap::{Parser};
use crate::models::LedgerEntry;
use crate::ledger::{append_entry, get_last_hash, read_all_entries};


#[derive(Parser)]
#[command(name="MiniQLDB", version="0.1", about="Lightweight, imutable ledger")]
pub struct cli{
    #[command(subcommand)]
    pub command:Commands,
}

pub enum Commands{
    Add{
        
    },
    Get{

    },
    History{

    },
}