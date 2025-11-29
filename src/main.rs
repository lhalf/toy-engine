mod transaction;

use crate::transaction::Transaction;
use anyhow::Context;

const BUFFER_CAPACITY: usize = 64 * 1024;

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).context("missing argument")?;

    let reader = csv::ReaderBuilder::new()
        .buffer_capacity(BUFFER_CAPACITY)
        .trim(csv::Trim::All)
        .from_path(&path)?;

    for transaction in reader.into_deserialize::<Transaction>() {
        println!("{:?}", transaction?);
    }

    Ok(())
}
