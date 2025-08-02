mod cli;
mod ledger;
mod models;
mod merkle;

fn main() {
    let entries= ledger::read_all_entries().unwrap();
    if entries.len() < 5 {
        println!("need 5 at least to build block")
    }
    let block = merkle::MerkleBlock::new(entries[..5].to_vec());
    println!("Merkle Root Created \\o/");
    println!("Merkle Root: {}", block.merkle_root);
    // cli::run();
}
