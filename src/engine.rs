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
impl Engine {
    fn available_and_held_for_client(&self, client_id: ClientID) -> (f64, f64) {
        use rust_decimal::prelude::ToPrimitive;

        let account = self.accounts.get(&client_id).unwrap();
        (
            account.available.to_f64().unwrap(),
            account
                .held_transactions
                .values()
                .sum::<Decimal>()
                .to_f64()
                .unwrap(),
        )
    }
}

#[cfg(test)]
mod test_deposit {
    use crate::engine::Engine;
    use crate::transaction::Transaction;

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
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn two_deposits_to_the_same_account() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        engine.handle_transaction(Transaction::deposit(1, 2, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((2.0, 0.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn two_deposits_to_separate_accounts() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        engine.handle_transaction(Transaction::deposit(2, 2, 1.0));
        assert_eq!(2, engine.accounts.len());
        for client in [1, 2] {
            assert_eq!((1.0, 0.0), engine.available_and_held_for_client(client));
        }
    }
}
#[cfg(test)]
mod test_withdrawal {
    use crate::engine::Engine;
    use crate::transaction::Transaction;
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
        assert_eq!((2.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::withdrawal(1, 2, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn withdrawal_over_available_does_nothing() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::withdrawal(1, 2, 2.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));
    }
}

#[cfg(test)]
mod test_dispute {
    use crate::engine::Engine;
    use crate::transaction::Transaction;
    #[test]
    fn deposit_and_dispute_reduces_available_and_increases_held() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn withdrawal_and_dispute_increases_available_and_decreases_held() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 2.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((2.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::withdrawal(1, 2, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 2));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((2.0, -1.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn disputing_a_non_existent_transaction_does_nothing() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 2));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn disputing_a_transaction_against_incorrect_client_does_nothing() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(2, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn transaction_can_only_be_disputed_once() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));
    }
}
