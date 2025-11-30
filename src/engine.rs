use crate::account::Account;
use crate::transaction::{ClientID, Transaction, TransactionID, TransactionType};
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Default)]
pub struct Engine {
    pub accounts: HashMap<ClientID, Account>,
}

impl Engine {
    pub fn handle_transaction(&mut self, transaction: Transaction) {
        match transaction {
            Transaction {
                r#type: TransactionType::Deposit,
                client,
                tx,
                amount: Some(amount),
            } => self.handle_deposit(client, tx, amount),
            Transaction {
                r#type: TransactionType::Withdrawal,
                client,
                tx,
                amount: Some(amount),
            } => self.handle_withdrawal(client, tx, amount),
            Transaction {
                r#type: TransactionType::Dispute,
                client,
                tx,
                amount,
            } if amount.is_none() => self.handle_dispute(client, tx),
            _ => todo!(),
        }
    }

    fn handle_deposit(
        &mut self,
        client_id: ClientID,
        transaction_id: TransactionID,
        amount: Decimal,
    ) {
        let account = self.accounts.entry(client_id).or_default();
        account.deposit(amount);
        account.transactions.insert(transaction_id, amount);
    }

    fn handle_withdrawal(
        &mut self,
        client_id: ClientID,
        transaction_id: TransactionID,
        amount: Decimal,
    ) {
        if let Some(account) = self.accounts.get_mut(&client_id) {
            account.withdraw(amount);
            account.transactions.insert(transaction_id, -amount);
        }
    }

    fn handle_dispute(&mut self, client_id: ClientID, transaction_id: TransactionID) {
        if let Some(account) = self.accounts.get_mut(&client_id) {
            account.dispute(transaction_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::Engine;
    use crate::transaction::Transaction;
    use rust_decimal::Decimal;
    use rust_decimal::prelude::FromPrimitive;

    #[test]
    fn no_deposits_creates_no_accounts() {
        let engine = Engine::default();
        assert!(engine.accounts.is_empty());
    }

    #[test]
    fn single_deposit_creates_single_account() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));

        assert_eq!(1, engine.accounts.len());
        let account = engine.accounts.get(&1).unwrap();
        assert_eq!(Decimal::from_f64(1.0).unwrap(), account.available);
        assert!(account.held.is_zero());
    }

    #[test]
    fn two_deposits_to_the_same_account() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));

        engine.handle_transaction(Transaction::deposit(1, 2, 1.0));

        assert_eq!(1, engine.accounts.len());
        let account = engine.accounts.get(&1).unwrap();
        assert_eq!(Decimal::from_f64(2.0).unwrap(), account.available);
        assert!(account.held.is_zero());
    }

    #[test]
    fn two_deposits_to_separate_accounts() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));

        engine.handle_transaction(Transaction::deposit(2, 2, 1.0));

        assert_eq!(2, engine.accounts.len());
        for client in [1, 2] {
            let account = engine.accounts.get(&client).unwrap();
            assert_eq!(Decimal::from_f64(1.0).unwrap(), account.available);
            assert!(account.held.is_zero());
        }
    }

    #[test]
    fn withdrawal_to_non_existent_client_does_not_create_account() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::withdrawal(1, 1, 1.0));

        assert!(engine.accounts.is_empty());
    }

    #[test]
    fn deposit_and_withdrawal_reduces_available() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 2.0));

        assert_eq!(1, engine.accounts.len());
        let mut account = engine.accounts.get(&1).unwrap();
        assert_eq!(Decimal::from_f64(2.0).unwrap(), account.available);
        assert!(account.held.is_zero());

        engine.handle_transaction(Transaction::withdrawal(1, 2, 1.0));

        assert_eq!(1, engine.accounts.len());
        account = engine.accounts.get(&1).unwrap();
        assert_eq!(Decimal::from_f64(1.0).unwrap(), account.available);
        assert!(account.held.is_zero());
    }

    #[test]
    fn withdrawal_over_available_does_nothing() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));

        assert_eq!(1, engine.accounts.len());
        let account = engine.accounts.get(&1).unwrap();
        assert_eq!(Decimal::from_f64(1.0).unwrap(), account.available);
        assert!(account.held.is_zero());

        engine.handle_transaction(Transaction::withdrawal(1, 2, 2.0));

        assert_eq!(1, engine.accounts.len());
        let account = engine.accounts.get(&1).unwrap();
        assert_eq!(Decimal::from_f64(1.0).unwrap(), account.available);
        assert!(account.held.is_zero());
    }

    #[test]
    fn deposit_and_dispute_reduces_available_and_increases_held() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));

        assert_eq!(1, engine.accounts.len());
        let mut account = engine.accounts.get(&1).unwrap();
        assert_eq!(Decimal::from_f64(1.0).unwrap(), account.available);
        assert!(account.held.is_zero());

        engine.handle_transaction(Transaction::dispute(1, 1));

        assert_eq!(1, engine.accounts.len());
        account = engine.accounts.get(&1).unwrap();
        assert!(account.available.is_zero());
        assert_eq!(Decimal::from_f64(1.0).unwrap(), account.held);
    }
}
