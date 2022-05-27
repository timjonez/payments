use crate::transactions::{Transaction, TransactionType};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl Account {
    pub fn new(id: u16) -> Account {
        Account {
            id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
    pub fn process(
        account: &mut Account,
        transaction: Transaction,
        transactions: &mut HashMap<u32, Transaction>,
    ) {
        match transaction.r#type {
            TransactionType::Deposit { amount } => {
                account.available += amount;
                transactions.insert(transaction.tx, transaction);
            }
            TransactionType::Withdrawal { amount } => {
                if amount <= account.available {
                    account.available -= amount;
                    transactions.insert(transaction.tx, transaction);
                } else {
                    eprintln!(
                        "Error: Unable to apply transaction with id of {}",
                        transaction.tx
                    );
                }
            }
            TransactionType::Dispute => {
                if transactions.contains_key(&transaction.tx) {
                    let disputed_transaction = &transactions.get(&transaction.tx).unwrap();
                    let amount = disputed_transaction.r#type.amount();
                    account.held += amount;
                    account.available -= amount;
                }
            }
            TransactionType::Resolve => {
                if transactions.contains_key(&transaction.tx) && account.held > 0.0 {
                    let disputed_transaction = &transactions.get(&transaction.tx).unwrap();
                    let amount = disputed_transaction.r#type.amount();
                    account.held -= amount;
                    account.available += amount;
                }
            }
            TransactionType::Chargeback => {
                if transactions.contains_key(&transaction.tx) && account.held > 0.0 {
                    let disputed_transaction = &transactions.get(&transaction.tx).unwrap();
                    let amount = disputed_transaction.r#type.amount();
                    account.held -= amount;
                    account.locked = true;
                }
            }
        }
        account.total = account.available + account.held;
    }
}
