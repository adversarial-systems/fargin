# Development Cycle Guidelines

## Project Quality Assurance Process

### 1. Formatting
Before committing any code, ensure proper formatting:
```bash
cargo fmt
```

### 2. Linting
Run Clippy to catch potential issues:
```bash
cargo clippy -- -D warnings
```

### 3. Testing
Run comprehensive test suite:
```bash
cargo test
```

### 4. Coverage
Check test coverage:
```bash
cargo tarpaulin
```

### 5. Security Audit
Check for dependency vulnerabilities:
```bash
cargo audit
```

### 6. Dependency Management
Check for outdated dependencies:
```bash
cargo outdated
```

## Git Hooks

Pre-commit hooks are configured to:
- Run `cargo fmt`
- Run `cargo clippy`

Pre-push hooks are configured to:
- Run `cargo test`

## Configuration Files

- `rustfmt.toml`: Formatting configuration
- `.clippy.toml`: Linting configuration
- `.fargin/dev_cycle.toml`: Development cycle settings

## Best Practices

1. Always run `./check.sh` before pushing changes
2. Keep test coverage above 70%
3. Address all warnings and linting issues
4. Regularly update dependencies
5. Conduct security audits

## Continuous Improvement

Periodically review and update:
- Development cycle configuration
- Tool configurations
- Testing strategies
