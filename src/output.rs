use crate::account::Account;
use crate::transaction::ClientID;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccountOutput {
    pub client: ClientID,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl From<(&ClientID, &Account)> for AccountOutput {
    fn from((client, account): (&ClientID, &Account)) -> Self {
        let held: Decimal = account.held_transactions.values().sum();
        Self {
            client: *client,
            available: account.available,
            held,
            total: account.available + held,
            locked: account.locked,
        }
    }
}
