# Deployment Guide

Memory Curator Agent is ready for GitHub and clawhub.ai. This guide covers both.

## GitHub Deployment

### 1. Create GitHub Repository

Go to https://github.com/new

**Repository name:** `memory-curator-agent`  
**Description:** "Automatically curate daily memory logs into MEMORY.md. Symbiont agent for OpenClaw."  
**Visibility:** Public  
**Initialize repo:** No (we have git history already)

### 2. Add Remote & Push

```bash
cd /home/jascha/.openclaw/workspace/memory-curator-agent

# Add your GitHub repo as origin
git remote add origin https://github.com/yourusername/memory-curator-agent.git
git branch -M main
git push -u origin main
```

### 3. Add GitHub Topics

On GitHub repo page → Settings → Topics:
```
memory-management
agent
symbiont
openclaw
rust
automation
curation
```

### 4. Enable GitHub Actions

Settings → Actions → General → Allow all actions

The CI/CD pipeline will automatically:
- Run `cargo test` on every push
- Run `cargo clippy` for linting
- Build release binary as artifact
- Verify formatting with `cargo fmt`

---

## clawhub.ai Deployment

### 1. Register on clawhub.ai

Visit https://clawhub.ai/register and create an account.

### 2. Submit the Skill

**Option A: Direct Upload**
1. Go to https://clawhub.ai/submit
2. Upload `clawhub.json`
3. The manifest includes everything clawhub needs

**Option B: GitHub Integration**
1. Go to https://clawhub.ai/connect-github
2. Authorize your GitHub account
3. clawhub will automatically sync on releases

### 3. Manifest Details

The `clawhub.json` includes:
- **Metadata:** Name, version, description, author
- **Capabilities:** What the agent does
- **Configuration:** User-settable options
- **Requirements:** Dependencies (Rust 1.70+, Symbiont 0.6.1+)
- **Installation method:** Direct GitHub clone
- **Security model:** Sandbox, audit, no secrets
- **Support links:** Issues, discussions, docs

### 4. Publishing Checklist

Before submitting to clawhub:

- [x] `clawhub.json` is valid JSON
- [x] Repository is public on GitHub
- [x] README.md is comprehensive
- [x] SKILL.md explains OpenClaw usage
- [x] License is clear (MIT)
- [x] Examples are included
- [x] Build/test commands documented
- [x] Security model documented
- [x] Changelog included

### 5. Approval Process

clawhub.ai team will:
1. Review `clawhub.json` and README
2. Test the build and basic functionality
3. Verify security sandbox requirements
4. Check documentation quality
5. Approve or request changes

Typical timeline: 24-48 hours.

---

## Post-Deployment

### Monitor Releases

Once on clawhub, users can install with:
```bash
openclaw skill install memory-curator
```

### GitHub Releases

To create a formal release:

```bash
# Tag a release
git tag -a v0.1.0 -m "Memory Curator Agent v0.1.0"
git push origin v0.1.0
```

Go to https://github.com/yourusername/memory-curator-agent/releases and:
1. Click "Draft a new release"
2. Select tag `v0.1.0`
3. Add release notes
4. Publish

### Update clawhub.json for New Versions

When releasing v0.2.0:

```json
{
  "version": "0.2.0",
  "changelog": {
    "0.2.0": {
      "date": "2026-MM-DD",
      "changes": [
        "Multi-file support",
        "Bug fixes"
      ]
    },
    "0.1.0": { ... }
  }
}
```

Push to GitHub. clawhub will auto-sync.

---

## Troubleshooting

### GitHub Push Fails

```bash
# Verify remote
git remote -v

# Reset remote if needed
git remote remove origin
git remote add origin https://github.com/yourusername/memory-curator-agent.git

# Push again
git push -u origin main
```

### GitHub Actions CI Fails

Check `.github/workflows/ci.yml`:
- Rust toolchain installed?
- Dependencies available?
- Tests passing locally?

Run locally to debug:
```bash
cargo test --verbose
cargo clippy
cargo fmt --check
```

### clawhub.ai Won't Accept Manifest

Validate JSON:
```bash
cat clawhub.json | jq .
```

Common issues:
- Malformed JSON
- Invalid URL in repository field
- Missing required fields (see manifest template)

### Downloads Not Showing

clawhub caches metadata. After publishing:
1. Wait 5-10 minutes
2. Clear browser cache
3. Check https://clawhub.ai/memory-curator

---

## Marketing & Sharing

### GitHub README Badge

```markdown
[![GitHub release](https://img.shields.io/github/release/yourusername/memory-curator-agent)](https://github.com/yourusername/memory-curator-agent/releases)
[![Build Status](https://github.com/yourusername/memory-curator-agent/workflows/CI/badge.svg)](https://github.com/yourusername/memory-curator-agent/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
```

### clawhub Badge

Once approved on clawhub:
```markdown
[![clawhub](https://img.shields.io/badge/clawhub-approved-brightgreen)](https://clawhub.ai/memory-curator)
```

### Share on Moltbook

Once published, share on Moltbook:
> "Memory Curator Agent is now on clawhub.ai! Automatically curate your daily memory logs into long-term memory. Built for agents that learn and grow. https://clawhub.ai/memory-curator"

---

## Support Going Forward

### Maintenance

After release, monitor:
- GitHub Issues → respond to bug reports
- clawhub reviews → address user feedback
- Symbiont/OpenClaw updates → ensure compatibility

### Future Versions

Plan updates with GitHub milestones:
```bash
git checkout -b feature/v0.2-multi-file
# ... make changes ...
git push origin feature/v0.2-multi-file
# Create PR on GitHub
```

---

## You're Ready! 🚀

**Next steps:**
1. Run `git remote add origin <your-github-url>`
2. Push to GitHub
3. Submit to clawhub.ai via https://clawhub.ai/submit
4. Share the release

Your Memory Curator Agent is production-ready.
