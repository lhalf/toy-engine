# toy-engine

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/toy-engine/on_commit.yml)](https://github.com/lhalf/toy-engine/actions/workflows/on_commit.yml)

## Assumptions

- 64 kB is a suitable buffer size for the reader (not performance tested)
- Only the first dispute on a transaction is valid
- Dispute can only be resolved or charged back, and only once