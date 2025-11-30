use rust_decimal::Decimal;
#[cfg(test)]
use rust_decimal::prelude::FromPrimitive;
use serde::Deserialize;

pub type ClientID = u16;
pub type TransactionID = u32;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub r#type: TransactionType,
    pub client: ClientID,
    pub tx: TransactionID,
    pub amount: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[cfg(test)]
impl Transaction {
    pub fn deposit(client: ClientID, tx: TransactionID, amount: f64) -> Self {
        Self {
            r#type: TransactionType::Deposit,
            client,
            tx,
            amount: Some(Decimal::from_f64(amount).unwrap()),
        }
    }

    pub fn withdrawal(client: ClientID, tx: TransactionID, amount: f64) -> Self {
        Self {
            r#type: TransactionType::Withdrawal,
            client,
            tx,
            amount: Some(Decimal::from_f64(amount).unwrap()),
        }
    }

    pub fn dispute(client: ClientID, tx: TransactionID) -> Self {
        Self {
            r#type: TransactionType::Dispute,
            client,
            tx,
            amount: None,
        }
    }

    pub fn resolve(client: ClientID, tx: TransactionID) -> Self {
        Self {
            r#type: TransactionType::Resolve,
            client,
            tx,
            amount: None,
        }
    }

    pub fn chargeback(client: ClientID, tx: TransactionID) -> Self {
        Self {
            r#type: TransactionType::Chargeback,
            client,
            tx,
            amount: None,
        }
    }
}
