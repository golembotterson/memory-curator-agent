# Memory Curator Agent

A Symbiont-based agent that maintains your long-term memory by curating daily logs into actionable insights.

## Overview

Memory Curator runs on a schedule (every 2-3 days) to:
1. **Scan** recent daily log files (`memory/YYYY-MM-DD.md`)
2. **Identify** signal: decisions, lessons learned, patterns, projects, important context
3. **Update** `MEMORY.md` with what's worth keeping long-term
4. **Prune** outdated entries no longer relevant
5. **Report** what changed and why

## Behavior

- **Input:** Path to memory directory, number of days to review
- **Output:** Updated MEMORY.md + change summary
- **Policy:** Read-only on daily files; write to MEMORY.md only
- **Audit:** All changes logged with reasoning
- **Safety:** No external API calls; operates on local files only

## Architecture

```
memory-curator-agent/
├── README.md (this file)
├── agent.dsl (Symbiont agent definition)
├── src/
│   ├── curator.rs (core logic)
│   ├── lib.rs
│   └── main.rs
├── Cargo.toml
├── .gitignore
└── examples/
    └── sample-config.json
```

## Quick Start

### Run Standalone (Rust)

```bash
cargo run -- \
  --memory-dir /path/to/memory \
  --days 2 \
  --output-report curator-report.json
```

### Run via Symbiont

First install symbi: `cargo install symbi`

```bash
symbi run agent.dsl --config config.json
```

### Run on Schedule (OpenClaw Cron)

Define a cron job to invoke daily:
```json
{
  "kind": "cron",
  "expr": "0 3 * * 2,5",
  "tz": "America/Los_Angeles",
  "payload": {
    "kind": "agentTurn",
    "message": "Curator: Review last 2 days of memory logs and update MEMORY.md"
  }
}
```

## Configuration

`config.json`:
```json
{
  "memory_dir": "/path/to/memory",
  "days_to_review": 2,
  "memory_file": "/path/to/MEMORY.md",
  "max_daily_entries": 5,
  "min_signal_confidence": 0.7,
  "prune_threshold_days": 90
}
```

## Security Model

- **Sandboxed:** Runs in isolated container (Symbiont Tier1/Tier2)
- **Policy-Bounded:** Only reads from `memory/` and writes to `MEMORY.md`
- **Audit Trail:** All mutations logged with Ed25519 signatures
- **No Secrets:** Operates on plaintext; no credential access
- **Error Recovery:** Fails safe; never corrupts source files

## Output

After each run:
- Updated `MEMORY.md` with new entries and pruned stale ones
- `curator-report.json` with:
  - Files scanned
  - Entries added (with confidence scores)
  - Entries removed (with reason)
  - Timestamp and agent signature

## Example Report

```json
{
  "timestamp": "2026-02-03T17:30:00Z",
  "agent_id": "memory-curator-v1",
  "files_scanned": ["memory/2026-02-03.md", "memory/2026-02-02.md"],
  "additions": [
    {
      "section": "Active Projects",
      "entry": "Memory Curator Agent - now in v1 development",
      "source_file": "memory/2026-02-03.md",
      "confidence": 0.95
    }
  ],
  "removals": [
    {
      "section": "TODO",
      "entry": "Review old Moltbook drafts",
      "reason": "completed_and_pruned_after_90_days",
      "last_referenced": "2025-11-15"
    }
  ]
}
```

## Development

Build:
```bash
cargo build --release
```

Test:
```bash
cargo test
```

Lint:
```bash
cargo clippy
```

## Future Features

- [ ] Semantic clustering (group similar entries)
- [ ] Cross-linking (detect related topics across files)
- [ ] Trend detection (identify recurring patterns)
- [ ] Multi-user support (separate memory domains)
- [ ] OpenClaw channel integration (post summaries to Slack/Discord)

## License

MIT

## Contributing

Submit PRs to the GitHub repo. All changes subject to audit trail review.
