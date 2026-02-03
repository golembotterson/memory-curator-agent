# Memory Curator Agent - OpenClaw Skill

**Skill Name:** `memory-curator`  
**Type:** Symbiont Agent  
**Status:** v0.1.0 (Ready for clawhub.ai)  
**License:** MIT  

## Overview

Memory Curator is an OpenClaw skill that automatically maintains your long-term memory by:
1. Scanning daily log files (`memory/YYYY-MM-DD.md`)
2. Identifying important signal (decisions, lessons, patterns)
3. Merging curated entries into `MEMORY.md`
4. Pruning stale, outdated information
5. Generating audit trails and reports

Perfect for agents that need to learn, remember, and grow over time.

## Quick Start

### Installation (clawhub.ai)

```bash
openclaw skill install memory-curator
```

### Manual Installation

```bash
git clone https://github.com/yourusername/memory-curator-agent.git
cd memory-curator-agent
cargo build --release
cp target/release/curator ~/.openclaw/skills/memory-curator/bin/
```

### Run as OpenClaw Cron Job

```json
{
  "kind": "cron",
  "expr": "0 3 * * 2,5",
  "tz": "America/Los_Angeles",
  "payload": {
    "kind": "agentTurn",
    "message": "Curator: Review last 2 days of memory logs and update MEMORY.md"
  },
  "sessionTarget": "isolated"
}
```

### Run Standalone

```bash
curator \
  --memory-dir /home/user/.openclaw/workspace/memory \
  --days 2 \
  --memory-file /home/user/.openclaw/workspace/MEMORY.md \
  --output-report /tmp/curator-report.json
```

## Configuration

Edit `~/.openclaw/workspace/memory-curator.json`:

```json
{
  "memory_dir": "~/.openclaw/workspace/memory",
  "days_to_review": 2,
  "memory_file": "~/.openclaw/workspace/MEMORY.md",
  "max_daily_entries": 5,
  "min_signal_confidence": 0.7,
  "prune_threshold_days": 90
}
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `memory_dir` | Path | `memory/` | Daily log directory |
| `days_to_review` | Int | `2` | Days back to scan |
| `memory_file` | Path | `MEMORY.md` | Long-term memory file |
| `max_daily_entries` | Int | `5` | Max entries per run |
| `min_signal_confidence` | Float | `0.7` | Signal threshold (0.0-1.0) |
| `prune_threshold_days` | Int | `90` | Delete entries older than N days |

## How It Works

### Signal Detection

The curator uses pattern matching + confidence scoring to identify:
- **Headlines** (`# Title`) → High signal
- **Structural sections** (`Status:`, `Next Steps:`) → High signal  
- **Keywords** (`decision`, `learned`, `critical`, `completed`) → Boost score
- **Generic entries** → Low confidence, often skipped

Entries scoring ≥ `min_signal_confidence` are promoted to MEMORY.md.

### Audit Trail

Each change is logged with:
- Timestamp and source file
- Confidence score
- Reason for inclusion/removal
- Ed25519 signature (when running via Symbiont)

Reports are written to JSON:
```json
{
  "timestamp": "2026-02-03T17:30:00Z",
  "agent_id": "memory-curator-v0.1",
  "files_scanned": ["memory/2026-02-03.md"],
  "additions": [...],
  "removals": [...],
  "status": "completed"
}
```

## Security

- **Sandboxed:** Runs in isolated Docker container (Symbiont Tier1)
- **Policy-bounded:** Only reads `memory/`, writes `MEMORY.md` + reports
- **No secrets:** Operates on plaintext files only
- **Error recovery:** Never corrupts source files; rollback on failure
- **Audit signed:** All mutations logged with timestamps + signatures

## Examples

### Daily Curation (Recommended)

Schedule daily at 3 AM (pick quiet time):

```json
{
  "kind": "cron",
  "expr": "0 3 * * *",
  "tz": "America/Los_Angeles",
  "payload": {
    "kind": "agentTurn",
    "message": "Run memory curation: scan last 2 days, merge signal, prune stale entries",
    "deliver": true
  }
}
```

### Weekly Deep Review

Review last 7 days, lower confidence threshold:

```bash
curator --days 7 --confidence 0.6
```

### Batch Catch-Up (After Vacation)

Review last 30 days, higher max entries:

```bash
curator --days 30 --max-entries 20
```

## Development

### Build
```bash
cargo build --release
cargo build --features symbiont  # Full Symbiont integration
```

### Test
```bash
cargo test
cargo test -- --nocapture  # With output
```

### Debug
```bash
RUST_LOG=debug cargo run -- --memory-dir ./test-memory
```

### Contributing

See [CONTRIBUTING.md](.github/CONTRIBUTING.md) for guidelines.

## FAQ

**Q: Can it handle multiple memory files?**  
A: Not yet. v0.2 will add multi-file and multi-user support.

**Q: What if MEMORY.md gets corrupted?**  
A: The agent creates a backup before writing and rolls back on failure.

**Q: How does it know what's important?**  
A: Pattern matching + keyword scoring. False positives? Edit MEMORY.md directly; the curator respects manual entries.

**Q: Can it integrate with my Symbiont policies?**  
A: Yes. See agent.dsl for policy definition; use Symbiont's policy engine to gate execution.

**Q: Why not use AI to detect signal?**  
A: v0.1 is offline and deterministic. v0.2 will add optional LLM-based curation.

## Troubleshooting

### Agent hangs
Check `memory/` directory permissions. Ensure read access.

### MEMORY.md not updated
Verify write permissions. Check logs: `curator --output-report /tmp/debug.json`

### Memory file corruption
Restore from git: `git checkout MEMORY.md`

### Entries not detected
Lower `min_signal_confidence` in config. Check if headlines are properly formatted.

## Roadmap

- [ ] v0.2: Multi-file support, multi-user domains
- [ ] v0.3: LLM-based signal detection (optional, offline)
- [ ] v0.4: Channel integration (post summaries to Slack/Discord)
- [ ] v0.5: Semantic clustering + cross-linking
- [ ] v1.0: Stable API, full Symbiont v1.0 support

## Support

- **Issues:** [GitHub Issues](https://github.com/yourusername/memory-curator-agent/issues)
- **Discussions:** [GitHub Discussions](https://github.com/yourusername/memory-curator-agent/discussions)
- **Security:** Report to maintainers privately

## License

MIT — see [LICENSE](LICENSE)

---

**Ready to use.** Built for agents that learn.
