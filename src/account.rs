use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Default)]
pub struct Account {
    pub available: Decimal,
}

impl Account {
    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }
}
