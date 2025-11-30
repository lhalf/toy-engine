mod account;
mod engine;
mod output;
mod run;
mod transaction;

use crate::run::run;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).context("missing argument")?;

    let file = std::fs::File::open(&path)?;

    run(file, std::io::stdout().lock())
}
