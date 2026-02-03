# Setup & Build Guide

## Prerequisites

- **Rust 1.70+** — Install from https://rustup.rs/
- **Git** — For version control
- **OpenClaw** (optional) — For Symbiont integration
- **Symbiont 0.6.1+** (optional) — For sandboxed execution

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/memory-curator-agent.git
cd memory-curator-agent
```

### 2. Build Release Binary

```bash
cargo build --release
```

Output: `target/release/curator`

### 3. Install Binary (Optional)

```bash
# Copy to system bin
sudo cp target/release/curator /usr/local/bin/

# Or install to OpenClaw skills
mkdir -p ~/.openclaw/skills/memory-curator/bin
cp target/release/curator ~/.openclaw/skills/memory-curator/bin/
```

## Usage

### Standalone CLI

```bash
curator --help
```

Options:
```
--memory-dir <PATH>     Path to memory directory (default: ~/.openclaw/workspace/memory)
--days <N>              Days back to review (default: 2)
--memory-file <PATH>    Path to MEMORY.md file (default: ~/.openclaw/workspace/MEMORY.md)
--output-report <PATH>  Report output path (default: /tmp/curator-report.json)
--confidence <N>        Confidence threshold 0.0-1.0 (default: 0.7)
--max-entries <N>       Max entries per run (default: 5)
```

### Example Runs

Daily curation (last 2 days):
```bash
curator
```

Deep review (last week, lower threshold):
```bash
curator --days 7 --confidence 0.6
```

Catch-up after vacation (30 days, more entries):
```bash
curator --days 30 --max-entries 20 --output-report vacation-review.json
```

### Via Symbiont

```bash
symbi run agent.dsl --config examples/sample-config.json
```

### Via OpenClaw Cron

Add to your cron config:
```json
{
  "kind": "cron",
  "expr": "0 3 * * 2,5",
  "tz": "America/Los_Angeles",
  "payload": {
    "kind": "agentTurn",
    "message": "Run memory curation",
    "agentId": "memory-curator"
  }
}
```

## Development

### Build Modes

Debug build (faster):
```bash
cargo build
```

Release build (optimized):
```bash
cargo build --release
```

With Symbiont features:
```bash
cargo build --release --features symbiont
```

### Testing

Run all tests:
```bash
cargo test
```

Run with output:
```bash
cargo test -- --nocapture
```

Run specific test:
```bash
cargo test test_extract_headlines
```

### Formatting & Linting

Check code style:
```bash
cargo fmt --check
```

Auto-fix:
```bash
cargo fmt
```

Lint:
```bash
cargo clippy
```

Lint strict:
```bash
cargo clippy -- -D warnings
```

### Debugging

Run with debug logging:
```bash
RUST_LOG=debug cargo run -- --memory-dir ./test-memory
```

Generate detailed traces:
```bash
RUST_BACKTRACE=1 cargo run
```

## Project Structure

```
memory-curator-agent/
├── src/
│   ├── main.rs          CLI entry point (clap args)
│   ├── lib.rs           Library root
│   ├── curator.rs       Core curation logic
│   ├── models.rs        Data structures
│   └── error.rs         Error types
├── agent.dsl            Symbiont agent definition
├── Cargo.toml           Rust dependencies
├── Cargo.lock           Lock file (version-pinned deps)
├── README.md            User documentation
├── SKILL.md             OpenClaw skill docs
├── SETUP.md             This file
├── clawhub.json         clawhub.ai manifest
├── examples/
│   └── sample-config.json
├── .github/
│   ├── workflows/ci.yml GitHub Actions CI
│   └── CONTRIBUTING.md  Contribution guidelines
├── LICENSE              MIT license
└── .gitignore
```

## Troubleshooting

### Build Fails: "error: could not compile..."

**Check Rust version:**
```bash
rustc --version
```
Must be 1.70+. Upgrade:
```bash
rustup update
```

**Clean and rebuild:**
```bash
cargo clean
cargo build --release
```

### Tests Fail

Create test memory directory:
```bash
mkdir -p test-memory
echo "# Test Entry" > test-memory/2026-02-03.md
```

Re-run tests:
```bash
cargo test
```

### Binary Not Found After Build

Check path:
```bash
ls -la target/release/curator
```

If missing, rebuild:
```bash
cargo build --release 2>&1 | tail -20
```

### Permission Denied When Writing MEMORY.md

Check file permissions:
```bash
ls -la ~/.openclaw/workspace/MEMORY.md
```

Fix:
```bash
chmod 644 ~/.openclaw/workspace/MEMORY.md
```

## Next Steps

1. **Build the binary:** `cargo build --release`
2. **Run tests:** `cargo test`
3. **Try it:** `./target/release/curator --help`
4. **Schedule it:** Add to your cron config or OpenClaw
5. **Contribute:** Submit PRs to improve signal detection

## Support

- **Docs:** See [README.md](README.md) and [SKILL.md](SKILL.md)
- **Issues:** [GitHub Issues](https://github.com/yourusername/memory-curator-agent/issues)
- **Questions:** [GitHub Discussions](https://github.com/yourusername/memory-curator-agent/discussions)
