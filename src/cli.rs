use clap::{Parser,Subcommand};
use crate::models::LedgerEntry;
use crate::ledger::{append_entry, get_last_hash, read_all_entries};


#[derive(Parser)]
#[command(name="MiniQLDB", version="0.1", about="Lightweight, imutable ledger")]
pub struct Cli{
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands{
    Add{
        id:String,
        json_data:String,
    },
    Get{
        id:String,
    },
    History{
        id:String,
    },
}

pub fn run(){
    let cli = Cli::parse();
    match cli.command{
        Commands::Add { id, json_data } =>{
            let data: serde_json::Value = serde_json::from_str(&json_data).expect("Invalid JSON Format");
            let prev_hash = get_last_hash().unwrap();
            let entry = LedgerEntry::new(id.clone(),data,prev_hash);
            append_entry(&entry).expect("Failed to write ledger");
            println!("Added ID: {}",id);
        }
        Commands::Get { id } =>{
            let entries= read_all_entries().unwrap();
            let latest = entries.iter().find(|e| e.id==id);
            match latest {
                Some(e)=>println!("{:#?}",e),
                None => println!("No entry found for ID: {}",id)
            }
        }
        Commands::History { id }=>{
            let entries=read_all_entries().unwrap();
            let history: Vec<_> = entries.into_iter().filter(|e| e.id==id).collect();
            if history.is_empty(){
                println!("No histroy found for ID: {}",id);
            }else{
                for entry in history{
                    println!("{:#?}", entry);
                }
            }
        }
    }
}