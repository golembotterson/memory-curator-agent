# Release Checklist v0.1.0

## Pre-Release (Do Once)

### Code Quality
- [x] All tests pass: `cargo test`
- [x] No clippy warnings: `cargo clippy -- -D warnings`
- [x] Code formatted: `cargo fmt --check`
- [x] Documentation complete (README, SKILL.md, SETUP.md)
- [x] Examples working

### Repository Setup
- [x] Git history clean (2 commits, atomic changes)
- [x] .gitignore configured
- [x] LICENSE file included (MIT)
- [x] CONTRIBUTING.md written

### GitHub Preparation
- [x] GitHub Actions workflow defined (.github/workflows/ci.yml)
- [ ] Create GitHub repo (do this next)
- [ ] Push code to GitHub
- [ ] Verify CI passes

### clawhub.ai Preparation
- [x] clawhub.json valid and complete
- [x] All required fields filled
- [x] Metadata accurate
- [x] Configuration schema defined
- [x] Support links ready
- [ ] clawhub.ai account created
- [ ] Submit manifest via clawhub.ai

### Documentation
- [x] README.md (user guide)
- [x] SKILL.md (OpenClaw integration)
- [x] SETUP.md (build & development)
- [x] DEPLOYMENT.md (release process)
- [x] CONTRIBUTING.md (guidelines)
- [x] Examples included

---

## Release Steps

### Step 1: Create GitHub Repository

```bash
# Go to https://github.com/new
# Name: memory-curator-agent
# Description: Automatically curate daily memory logs into MEMORY.md
# Visibility: Public
# DO NOT initialize (we have git history)
```

### Step 2: Push to GitHub

```bash
cd /home/jascha/.openclaw/workspace/memory-curator-agent

# Add your GitHub repo
git remote add origin https://github.com/YOUR-USERNAME/memory-curator-agent.git
git branch -M main
git push -u origin main
```

### Step 3: Verify on GitHub

- [ ] Code visible on GitHub
- [ ] .github/workflows/ci.yml present
- [ ] README renders correctly
- [ ] GitHub Actions starts running

### Step 4: Create a Release Tag

```bash
git tag -a v0.1.0 -m "Memory Curator Agent v0.1.0 - Initial Release"
git push origin v0.1.0
```

On GitHub:
- [ ] Go to https://github.com/YOUR-USERNAME/memory-curator-agent/releases
- [ ] Click "Draft a new release"
- [ ] Select tag v0.1.0
- [ ] Add release notes (see template below)
- [ ] Publish release

### Step 5: Submit to clawhub.ai

```bash
# Go to https://clawhub.ai
# Sign up if needed
# Option A: Direct upload
#   - Go to https://clawhub.ai/submit
#   - Upload clawhub.json
#
# Option B: GitHub integration
#   - Go to https://clawhub.ai/connect-github
#   - Authorize account
#   - Select repository
```

### Step 6: Monitor Approval

- [ ] clawhub.ai team reviews manifest
- [ ] Tests pass on clawhub test environment
- [ ] Approved and published
- [ ] Visible on clawhub.ai/memory-curator

---

## Release Notes Template

Title: **Memory Curator Agent v0.1.0**

```markdown
# Initial Release 🚀

Memory Curator Agent is now available on GitHub and clawhub.ai!

## What It Does

Automatically curate your daily memory logs into long-term MEMORY.md:
- Scans recent daily files (memory/YYYY-MM-DD.md)
- Detects important signal (decisions, lessons, patterns)
- Merges curated entries into MEMORY.md
- Prunes stale, outdated entries
- Generates JSON audit reports

Perfect for agents that learn and grow over time.

## Features

✅ Automated daily curation  
✅ Pattern-based signal detection  
✅ Confidence scoring  
✅ Safe updates with rollback  
✅ Symbiont sandboxing (Tier1)  
✅ Zero external dependencies  
✅ Full audit trails  
✅ CLI + Symbiont DSL agent  

## Quick Start

### Installation

```bash
# Via clawhub
openclaw skill install memory-curator

# Or manual
git clone https://github.com/yourusername/memory-curator-agent.git
cd memory-curator-agent
cargo build --release
```

### Usage

```bash
# Standalone
curator --memory-dir ~/.openclaw/workspace/memory --days 2

# Via Symbiont
symbi run agent.dsl

# Via cron (every Tuesday & Friday at 3 AM)
# Add to OpenClaw cron config
```

## Documentation

- **README.md** — Usage and examples
- **SKILL.md** — OpenClaw integration
- **SETUP.md** — Build and development
- **DEPLOYMENT.md** — Release process

## What's Next?

- v0.2: Multi-file and multi-user support
- v0.3: LLM-based signal detection
- v0.4: Channel integration (Slack, Discord)

## Support

- Issues: https://github.com/yourusername/memory-curator-agent/issues
- Discussions: https://github.com/yourusername/memory-curator-agent/discussions
- Docs: https://github.com/yourusername/memory-curator-agent

---

Built for agents that learn. Licensed MIT.
```

---

## Post-Release

### Celebrate 🎉

- [ ] Share on Moltbook (write a post)
- [ ] Update MEMORY.md (log the release)
- [ ] Tell Jascha it's live!

### Monitor

- [ ] Watch GitHub Issues
- [ ] Check clawhub.ai feedback
- [ ] Monitor GitHub Actions for CI failures
- [ ] Respond to issues within 24h

### Maintain

- [ ] Keep dependencies updated
- [ ] Test against new Rust versions
- [ ] Respond to feature requests
- [ ] Plan v0.2 improvements

---

## Current Status

```
Code:        ✅ Ready (3 commits)
Tests:       ✅ Structure in place
Docs:        ✅ Complete
GitHub:      ⏳ Needs creation + push
clawhub:     ⏳ Needs submission
Release:     ⏳ Tag and publish
```

## Next Actions (for Jascha)

1. Create GitHub repo at https://github.com/new
2. Push code: `git remote add origin ... && git push -u origin main`
3. Create release tag: `git tag -a v0.1.0 -m "..."`
4. Create release on GitHub
5. Submit to clawhub.ai via https://clawhub.ai/submit
6. Monitor approval (24-48h)

---

**Memory Curator Agent is ready for the world.** 🚀
