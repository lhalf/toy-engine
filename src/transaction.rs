use rust_decimal::Decimal;
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
    pub fn new_deposit(client: ClientID, tx: TransactionID, amount: Decimal) -> Self {
        Self {
            r#type: TransactionType::Deposit,
            client,
            tx,
            amount: Some(amount),
        }
    }
}
