use crate::engine::Engine;
use crate::transaction::Transaction;
use std::io::{Read, Write};

const BUFFER_CAPACITY: usize = 64 * 1024;

pub fn run(reader: impl Read, writer: impl Write) -> anyhow::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .buffer_capacity(BUFFER_CAPACITY)
        .trim(csv::Trim::All)
        .from_reader(reader);

    let mut engine = Engine::default();

    for record in reader.deserialize::<Transaction>() {
        engine.handle_transaction(record?);
    }

    let mut writer = csv::Writer::from_writer(writer);

    for row in engine.output() {
        writer.serialize(row)?;
    }

    writer.flush()?;

    Ok(())
}
