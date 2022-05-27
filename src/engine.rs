use crate::accounts::Account;
use crate::transactions::Transaction;
use csv::{Error, Reader};
use std::collections::HashMap;

pub fn run_transactions(args: &[String]) -> Result<(), Error> {
    let mut transactions = HashMap::new();
    let mut accounts = HashMap::new();

    let mut reader = Reader::from_path(&args[1])?;
    for result in reader.deserialize() {
        let record: Transaction = result?;
        process_transaction(record, &mut transactions, &mut accounts).unwrap();
    }
    println!("client, available, held, total, locked");
    for (_, account) in accounts.iter() {
        println!(
            "{},{:.4},{:.4},{:.4}, {}",
            account.id, account.available, account.held, account.total, account.locked
        );
    }
    Ok(())
}

fn process_transaction(
    record: Transaction,
    transactions: &mut HashMap<u32, Transaction>,
    accounts: &mut HashMap<u16, Account>,
) -> Result<(), String> {
    if !accounts.contains_key(&record.client) {
        let account = Account::new(record.client);
        accounts.insert(record.client, account);
    }
    let account = accounts.get_mut(&record.client).unwrap();
    Account::process(account, record, transactions);
    Ok(())
}
