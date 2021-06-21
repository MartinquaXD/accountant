mod account_manager;
mod helper_types;
mod transaction;

use crate::account_manager::AccountManager;
use crate::transaction::Transaction;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let input_file = args.get(1).ok_or("no input file argument passed")?;

    let mut account_manager = AccountManager::default();

    let mut reader = csv::Reader::from_path(input_file)?;
    for read_result in reader.records() {
        if let Ok(transaction) = Transaction::try_from(read_result?) {
            account_manager.handle_transaction(transaction);
        }
    }
    println!("{}", account_manager);
    Ok(())
}
