# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Structure

This is a multi-project workspace containing independent learning/demo projects:

- **cyclic-ca/** - Rust cellular automata simulation (eframe/egui, native + WASM)
- **stock-monitor/** - Python stock monitoring agent
- **simple-web/** - Static HTML demo page
- **misc-md_files_etc/** - Claude Code reference documentation and notes

## Build and Run Commands

### cyclic-ca (Rust)

```bash
cd cyclic-ca
cargo build              # Build
cargo run                # Run simulation
cargo build --release    # Release build
./build-macos-app.sh     # Build macOS app bundle (creates CyclicCA.app)
```

WASM/Browser build (requires trunk):
```bash
trunk build --release    # Build to dist/
trunk serve              # Dev server with hot reload
```

Controls: Click to start/restart, P to pause, R to restart

### stock-monitor (Python)

```bash
cd stock-monitor
pip install yfinance                    # Install dependency
python stock_agent.py                   # Single run
python stock_agent.py --continuous      # Run hourly
python scheduler.py --schedule          # Python scheduler (foreground)
python scheduler.py --install-launchd   # Install macOS launchd job
```

Configure watchlist by editing `WATCHLIST` in `stock_agent.py`.

## Architecture Notes

### cyclic-ca

Rust app using eframe/egui for cross-platform graphics (native + WASM). Multi-file architecture:
- `main.rs` - Entry point, platform-specific setup (native/WASM)
- `app.rs` - Main application state and eframe integration
- `ca.rs` - `CyclicCellularAutomata` struct, grid state and simulation logic
- `ui.rs` - UI controls and rendering

Each cell type "eats" the previous type in a cyclic chain (type N consumes type N-1, wrapping around).

### stock-monitor

Plugin-based agent architecture:
- `fetch_quote()` / `fetch_all()` - Data acquisition (yfinance)
- `check_alerts()` - Analysis (threshold comparison)
- `notify()` - Output (console/file, extensible)
- `StockAgent` class - Orchestration with `add_analyzer()` / `add_notifier()` hooks

Three scheduling options: cron, Python `schedule` library, or macOS launchd.

### misc-md_files_etc

Reference documentation folder containing:
- `claude-code-manual.md` - Claude Code usage manual
- `claude-code-reference.md` - Quick reference guide
- `keyboard-shortcut.txt` - Keyboard shortcuts reference
