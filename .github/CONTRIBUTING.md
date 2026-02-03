# Contributing

## Getting Started

1. Fork the repo
2. Clone your fork
3. Create a feature branch: `git checkout -b feature/your-feature`
4. Commit with clear messages
5. Push and open a PR

## Development

### Prerequisites
- Rust 1.70+
- Symbiont v0.6.1+
- OpenClaw (for testing with Symbiont)

### Build
```bash
cargo build --release
```

### Test
```bash
cargo test
```

### Lint
```bash
cargo clippy
```

## Code Style

- Follow Rust conventions (rustfmt)
- Document public APIs
- Add tests for new features
- Keep commits atomic and well-messaged

## Reporting Issues

Include:
- Rust/Symbiont versions
- Reproduction steps
- Expected vs actual behavior
- Environment (OS, architecture)

## Security

Report security issues privately to maintainers. Do not open public issues for vulnerabilities.
