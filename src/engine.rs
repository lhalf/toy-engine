use crate::account::Account;
use crate::output::AccountOutput;
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
            Transaction {
                r#type: TransactionType::Resolve,
                client,
                tx,
                amount,
            } if amount.is_none() => self.handle_resolve(client, tx),
            Transaction {
                r#type: TransactionType::Chargeback,
                client,
                tx,
                amount,
            } if amount.is_none() => self.handle_chargeback(client, tx),
            // currently ignore if the transaction is malformed
            _ => (),
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

    fn handle_resolve(&mut self, client_id: ClientID, transaction_id: TransactionID) {
        if let Some(account) = self.accounts.get_mut(&client_id) {
            account.resolve(transaction_id);
        }
    }

    fn handle_chargeback(&mut self, client_id: ClientID, transaction_id: TransactionID) {
        if let Some(account) = self.accounts.get_mut(&client_id) {
            account.chargeback(transaction_id);
        }
    }

    pub fn output(&self) -> impl Iterator<Item = AccountOutput> {
        self.accounts.iter().map(AccountOutput::from)
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

    fn is_account_locked_for_client(&self, client_id: ClientID) -> bool {
        self.accounts.get(&client_id).unwrap().locked
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

#[cfg(test)]
mod test_resolve {
    use crate::engine::Engine;
    use crate::transaction::Transaction;

    #[test]
    fn deposit_dispute_and_resolve_releases_held_funds() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::resolve(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn withdrawal_dispute_and_resolve_releases_held_funds() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::withdrawal(1, 2, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 2));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, -1.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::resolve(1, 2));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 0.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn resolving_a_non_existent_transaction_does_nothing() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::resolve(1, 2));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn resolving_a_transaction_against_incorrect_client_does_nothing() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::resolve(2, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));
    }

    #[test]
    fn dispute_can_only_be_resolved_once() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::resolve(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::resolve(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));
    }
}

#[cfg(test)]
mod test_chargeback {
    use crate::engine::Engine;
    use crate::transaction::Transaction;

    #[test]
    fn deposit_dispute_and_chargeback_releases_held_funds_and_locks_account() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::dispute(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((0.0, 1.0), engine.available_and_held_for_client(1));

        engine.handle_transaction(Transaction::chargeback(1, 1));
        assert_eq!(1, engine.accounts.len());
        assert_eq!((1.0, 0.0), engine.available_and_held_for_client(1));
        assert!(engine.is_account_locked_for_client(1));
    }
}
