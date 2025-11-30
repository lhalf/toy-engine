use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Default)]
pub struct Account {
    pub available: Decimal,
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
}
