# toy-engine

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/toy-engine/on_commit.yml)](https://github.com/lhalf/toy-engine/actions/workflows/on_commit.yml)

## Running commands

If you are in a Rust environment you can run commands by installing [just](https://just.systems/man/en/).

```bash
cargo install just
```

All the commands are available in the justfile in the top level, if you'd rather run them manually.

Use the -l flag to see all available commands and how to use them e.g. `just -l`.

## Assumptions

- 64 kB is a suitable buffer size for the reader (not performance tested)
- Only the first dispute on a transaction is valid
- Dispute can only be resolved or charged back, and only once
- Amounts can be negative, probably doesn't make sense!