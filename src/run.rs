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

#[cfg(test)]
mod tests {
    use crate::run::run;

    #[test]
    fn single_deposit() {
        let input = b"type,client,tx,amount\ndeposit,1,1,1.0\n";
        let mut output = Vec::new();
        let expected_output = b"client,available,held,total,locked\n1,1,0,1,false\n";

        assert!(run(&input[..], &mut output).is_ok());
        assert_eq!(output, expected_output);
    }
}
