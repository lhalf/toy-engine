use rust_decimal::Decimal;
#[cfg(test)]
use rust_decimal::prelude::FromPrimitive;
use serde::Deserialize;

pub type ClientID = u16;
pub type TransactionID = u32;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Transaction {
    #[serde(rename = "type")]
    pub r#type: TransactionType,
    pub client: ClientID,
    pub tx: TransactionID,
    pub amount: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
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

#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;
    use anyhow::Context;

    fn try_deserialize(csv: &str) -> anyhow::Result<Transaction> {
        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(csv.as_bytes());

        reader
            .deserialize::<Transaction>()
            .next()
            .context("more than one record")?
            .context("failed to deserialize transaction")
    }

    #[test]
    fn deposit() {
        let input = "\
type,client,tx,amount
deposit, 1, 10, 2.5000
";

        assert_eq!(
            Transaction::deposit(1, 10, 2.5),
            try_deserialize(input).unwrap()
        );
    }

    #[test]
    fn withdrawal() {
        let input = "\
type,client,tx,amount
withdrawal,2,20,1.2345
";

        assert_eq!(
            Transaction::withdrawal(2, 20, 1.2345),
            try_deserialize(input).unwrap()
        );
    }

    #[test]
    fn dispute() {
        let input = "\
type,client,tx,amount
dispute, 3, 30,
";

        assert_eq!(Transaction::dispute(3, 30), try_deserialize(input).unwrap());
    }

    #[test]
    fn resolve() {
        let input = "\
type,client,tx,amount
resolve,4,40,
";

        assert_eq!(Transaction::resolve(4, 40), try_deserialize(input).unwrap());
    }

    #[test]
    fn chargeback() {
        let input = "\
type,client,tx,amount
chargeback,5,50,
";

        assert_eq!(
            Transaction::chargeback(5, 50),
            try_deserialize(input).unwrap()
        );
    }

    #[test]
    fn whitespace() {
        let input = "\
type,client,tx,amount
  deposit , 9 , 90 , 10.0000
";

        assert_eq!(
            Transaction::deposit(9, 90, 10.0000),
            try_deserialize(input).unwrap()
        );
    }
}
