# Personal Finance CLI Manager

This project is a simple command-line application written in Rust for tracking personal income and expenses. It allows importing transactions from CSV files, storing them in a local SQLite database, adding manual entries, and generating basic monthly reports.

## Architecture (First Checkpoint)

The architecture is finalized and includes the following modules:

- `main.rs` – application entry point and command dispatch
- `cli.rs` – command-line interface (add, import, report)
- `models.rs` – data structures (Transaction)
- `db.rs` – SQLite connection and storage layer
- `import.rs` – CSV import logic
- `report.rs` – monthly reporting functionality
- `categorize.rs`, `budget.rs`, `search.rs`, `tui.rs` – empty modules that will be implemented later (architecture completed)

## Implemented Functionality (First Checkpoint)

The following features are already functional:

- Import transactions from a CSV file:
  ```bash
  cargo run -- --db finance.db import --file data/sample.csv
