# amaru-doctor

amaru-doctor is a Terminal User Interface (TUI) application written in Rust that provides an interface for inspecting Amaru blockchain data. It displays information about ledger accounts, chain data, and OpenTelemetry traces in an interactive terminal interface with mouse support.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Prerequisites and Environment Setup
- Install Rust nightly toolchain (nightly-2025-04-16): `rustup toolchain install nightly-2025-04-16`
- Set the toolchain: `rustup override set nightly-2025-04-16` 
- Required components are automatically installed via rust-toolchain.toml
- Set environment variables for database files:
  - `export AMARU_LEDGER_DB=resources/ledger.db`
  - `export AMARU_CHAIN_DB=resources/chain.db`

### Build and Development Commands
- **CRITICAL BUILD**: `cargo build --release` -- takes 13 minutes to complete. NEVER CANCEL. Set timeout to 20+ minutes. Builds may fail with network timeouts due to external Git dependencies but will retry automatically.
- **CRITICAL TESTS**: `cargo test --locked --all-features --workspace` -- takes 6 minutes. NEVER CANCEL. Set timeout to 15+ minutes.
- **FAST FORMATTING**: `cargo fmt --all --check` -- takes <1 second.
- **LINTING**: `cargo clippy --all-targets --all-features --workspace -- -D warnings` -- takes 6 minutes. NEVER CANCEL. Set timeout to 15+ minutes.

### Running the Application
- **REQUIREMENTS**: Application requires database files. Use provided sample files:
  - `AMARU_CHAIN_DB=resources/chain.db AMARU_LEDGER_DB=resources/ledger.db ./target/release/amaru-doctor`
- **HELP**: `./target/release/amaru-doctor --help` shows available options
- **INSTALLATION WARNING**: `cargo install --path .` currently fails due to dependency version conflicts between pallas crates. Use `cargo build --release` and run the binary directly from `./target/release/amaru-doctor`
- **QUITTING**: Press `q`, `Esc`, `Ctrl+d`, or `Ctrl+c` to quit the application
- **MOUSE SUPPORT**: Click widgets to focus them, mouse movement tracked for hover effects

## Validation

### Manual Testing Scenarios
- **ALWAYS**: After making changes, build the project and verify the binary runs without crashing
- **UI VALIDATION**: Run the application with sample databases and verify:
  - Application starts and displays the TUI interface correctly
  - Can navigate between different inspect options (ledger, chain, otel) using Shift+arrows
  - Can browse accounts, block issuers, DReps, pools, proposals, and UTXOs
  - Mouse clicking works to focus different widgets
  - Application responds to keyboard navigation (arrow keys, shift+arrows)
- **DATABASE INTEGRATION**: Verify application can read from both ledger.db and chain.db files
- **ERROR HANDLING**: Test with missing database files to ensure proper error messages

### Pre-commit Validation
- ALWAYS run `cargo fmt --all --check` -- formatting must pass or CI will fail
- ALWAYS run `cargo clippy --all-targets --all-features --workspace -- -D warnings` -- no warnings allowed or CI will fail
- ALWAYS run `cargo test --locked --all-features --workspace` -- all tests must pass

## Common Tasks

### Development Workflow
1. Make code changes
2. `cargo fmt --all` to format code
3. `cargo clippy --all-targets --all-features --workspace -- -D warnings` to check linting
4. `cargo test --locked --all-features --workspace` to run tests
5. `cargo build --release` to build release binary
6. Test application manually with sample databases

### Key File Locations
- Main application entry: `src/main.rs`
- TUI logic: `src/tui.rs`, `src/app.rs` 
- Configuration: `src/config.rs`, `.config/config.json5`
- Database integration: `src/detection.rs`, `src/store/`
- UI components: `src/ui/`, `src/view/`
- Sample databases: `resources/ledger.db`, `resources/chain.db`
- CI/CD workflows: `.github/workflows/ci.yml`, `.github/workflows/cd.yml`

### Cargo Commands Reference
```bash
# Format code
cargo fmt --all

# Check formatting (CI requirement)
cargo fmt --all --check

# Run linting (CI requirement) 
cargo clippy --all-targets --all-features --workspace -- -D warnings

# Run tests (CI requirement)
cargo test --locked --all-features --workspace

# Build release binary (RECOMMENDED)
cargo build --release

# Install from source (CURRENTLY BROKEN - use build instead)
# cargo install --path .

# Check documentation
cargo doc --no-deps --document-private-items --all-features --workspace --examples
```

### Environment Variables
- `AMARU_LEDGER_DB`: Path to ledger database file (required)
- `AMARU_CHAIN_DB`: Path to chain database file (required)  
- `AMARU_NETWORK`: Network name (default: preprod)
- `AMARU_DOCTOR_CONFIG`: Configuration directory (optional, default from .envrc: .config)
- `AMARU_DOCTOR_DATA`: Data directory (optional, default from .envrc: .data)
- `AMARU_DOCTOR_LOG_LEVEL`: Log level (optional, default from .envrc: debug)

### Repository Structure Summary
```
├── .github/workflows/     # CI/CD configuration
├── .config/              # Application configuration
├── resources/            # Sample database files and demo
├── src/                  # Rust source code
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Main application logic
│   ├── tui.rs           # Terminal UI management
│   ├── config.rs        # Configuration handling
│   └── ...              # Other modules
├── Cargo.toml           # Rust project configuration
├── rust-toolchain.toml  # Rust toolchain specification
└── build.rs             # Build script
```

### Troubleshooting
- **Build failures**: Usually due to network timeouts fetching Git dependencies. Wait for retries or run again.
- **Missing databases**: Application will panic if AMARU_LEDGER_DB or AMARU_CHAIN_DB are not set or files don't exist.
- **Nightly toolchain**: Project requires specific nightly Rust version defined in rust-toolchain.toml.
- **CI failures**: Check formatting (`cargo fmt --all --check`) and linting (`cargo clippy`) before committing.
- **cargo install fails**: Due to pallas dependency version conflicts. Use `cargo build --release` instead.

### Performance Expectations
- **Initial build**: 13+ minutes (downloads and compiles many dependencies)
- **Incremental builds**: Much faster, typically 1-2 minutes
- **Test suite**: 6 minutes (18 tests covering config parsing, mouse handling, etc.)
- **Linting**: 6 minutes (comprehensive clippy checks)
- **Formatting**: <1 second

NEVER CANCEL long-running operations. Build times of 10-15 minutes are normal for this project due to the large number of external dependencies from the Amaru ecosystem.