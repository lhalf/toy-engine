mod account;
mod engine;
mod output;
mod transaction;

use crate::engine::Engine;
use crate::transaction::Transaction;
use anyhow::Context;

const BUFFER_CAPACITY: usize = 64 * 1024;

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).context("missing argument")?;

    let reader = csv::ReaderBuilder::new()
        .buffer_capacity(BUFFER_CAPACITY)
        .trim(csv::Trim::All)
        .from_path(&path)?;

    let mut engine = Engine::default();

    for transaction in reader.into_deserialize::<Transaction>() {
        engine.handle_transaction(transaction?);
    }

    let mut writer = csv::Writer::from_writer(std::io::stdout());

    for row in engine.output() {
        writer.serialize(row)?;
    }

    writer.flush()?;

    Ok(())
}
