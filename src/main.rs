use anyhow::Context;

const BUFFER_CAPACITY: usize = 64 * 1024;

fn main() -> anyhow::Result<()> {
    let path = std::env::args().next().context("missing argument")?;

    let _reader = csv::ReaderBuilder::new()
        .buffer_capacity(BUFFER_CAPACITY)
        .trim(csv::Trim::All)
        .from_path(&path)?;

    Ok(())
}
