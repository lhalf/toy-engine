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
            _ => todo!(),
        }
    }

    fn handle_deposit(
        &mut self,
        client_id: ClientID,
        _transaction_id: TransactionID,
        amount: Decimal,
    ) {
        self.accounts.entry(client_id).or_default().deposit(amount);
    }

    fn handle_withdrawal(
        &mut self,
        client_id: ClientID,
        _transaction_id: TransactionID,
        amount: Decimal,
    ) {
        if let Some(account) = self.accounts.get_mut(&client_id) {
            account.withdraw(amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Account;
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
        assert_eq!(
            Account {
                available: Decimal::from_f64(1.0).unwrap(),
            },
            *engine.accounts.get(&1).unwrap()
        );
    }

    #[test]
    fn two_deposits_to_the_same_account() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));

        engine.handle_transaction(Transaction::deposit(1, 2, 1.0));

        assert_eq!(1, engine.accounts.len());
        assert_eq!(
            Account {
                available: Decimal::from_f64(2.0).unwrap(),
            },
            *engine.accounts.get(&1).unwrap()
        );
    }

    #[test]
    fn two_deposits_to_separate_accounts() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));

        engine.handle_transaction(Transaction::deposit(2, 2, 1.0));

        assert_eq!(2, engine.accounts.len());
        for client in [1, 2] {
            assert_eq!(
                Account {
                    available: Decimal::from_f64(1.0).unwrap(),
                },
                *engine.accounts.get(&client).unwrap()
            );
        }
    }

    #[test]
    fn deposit_and_withdrawal_reduces_available() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 2.0));

        assert_eq!(1, engine.accounts.len());
        assert_eq!(
            Account {
                available: Decimal::from_f64(2.0).unwrap(),
            },
            *engine.accounts.get(&1).unwrap()
        );

        engine.handle_transaction(Transaction::withdrawal(1, 2, 1.0));

        assert_eq!(1, engine.accounts.len());
        assert_eq!(
            Account {
                available: Decimal::from_f64(1.0).unwrap(),
            },
            *engine.accounts.get(&1).unwrap()
        );
    }

    #[test]
    fn withdrawal_over_available_does_nothing() {
        let mut engine = Engine::default();

        engine.handle_transaction(Transaction::deposit(1, 1, 1.0));

        assert_eq!(1, engine.accounts.len());
        assert_eq!(
            Account {
                available: Decimal::from_f64(1.0).unwrap(),
            },
            *engine.accounts.get(&1).unwrap()
        );

        engine.handle_transaction(Transaction::withdrawal(1, 2, 2.0));

        assert_eq!(1, engine.accounts.len());
        assert_eq!(
            Account {
                available: Decimal::from_f64(1.0).unwrap(),
            },
            *engine.accounts.get(&1).unwrap()
        );
    }
}
