use super::Transaction;
use std::sync::{Arc, Mutex};

pub type TransactionVec = Vec<Transaction>;

// We don't need to export this type because concurrency is encapsulated in this file
type SyncedTransactionVec = Ard<Mutex<TransactionVec>>;

// Represent a pool of unrealized transactions
// Multiple threads can read/ write concurrently to the pool
#[derive(Debug, Clone)]
pub struct TransactionPool {
    transactions: SyncedTransactionVec,
}

// Basic operations in the transaction pool are encapsulated in the implementation
// Encapsulates concurrency concerns, so external callers do not need to know how it's handled
impl TransactionPool {
    // Created an empty transaction pool
    pub fn new() -> TransactionPool {
        TransactionPool {
            transactions: SyncedTransactionVec::default(),
        }
    }

    // Adds a new transaction to the pool
    pub fn add_transaction(&self, transaction: Transaction) {
        // Transaction should be validated before being included in the pool
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction);
        info!("transaction added");
    }

    // Returns a copy of all transactions and empties the pool
    // This operation is safe to be called concurrently from multiple threads
    pub fn pop(&self) -> TransactionVec {
        // the "transactions" attribute is protected by a  Mutex
        // so only one thread at a time can access the value when the lock is held
        // preventing inconsistencies when adding new transactions while a pop is in course
        let mut transactions = self.transactions.lock().unwrap();
        let transactions_clone = transactions.clone();
        transactions.clear();

        transactions_clone
    }
}

#[cfg(test)]
mod tests {
    use crate::model::test_util::{alice, bob};

    use super::*;

    #[test]
    fn should_be_empty_after_creation() {

    }
}