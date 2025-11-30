use crate::transaction::TransactionID;
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Account {
    pub available: Decimal,
    pub transactions: HashMap<TransactionID, Decimal>,
    pub held_transactions: HashMap<TransactionID, Decimal>,
}

impl Account {
    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }
    pub fn withdraw(&mut self, amount: Decimal) {
        if self.available >= amount {
            self.available -= amount;
        }
    }
    pub fn dispute(&mut self, transaction: TransactionID) {
        if let Some(amount) = self.transactions.get(&transaction)
            && !self.held_transactions.contains_key(&transaction)
        {
            self.available -= *amount;
            self.held_transactions.insert(transaction, *amount);
        }
    }
}
