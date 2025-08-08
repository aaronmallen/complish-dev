# Contributing to Complish

Thank you for your interest in contributing to Complish! This guide will help you get started with development and
understand our contribution process.

## Quick Start

### Prerequisites

- Rust (see `rust-toolchain.toml` for version requirements)
- Git
- [Mise](https://mise.jdx.dev) for version management and task running _(recommended)_

### Setup

> [!NOTE]
> These instructions assume you are using mise. If you are not using mise, you can run the commands below by using
> the associated bin script directly.

1. Fork and clone the repository

   ```bash
   git clone https://github.com/your-username/complish-dev.git
   cd complish-dev
   ```

2. Install development dependencies

   ```bash
   mise run setup
   # Or without mise: ./bin/setup
   ```

3. Build the entire CI pipeline

   ```bash
   mise run ci
   # Or without mise: ./bin/ci
   ```

## Project Structure

This is a Rust workspace with the following crates:

> [!WARNING]
> This project is still in development and it's structure is subject to change.

```ls
crates/
├── complish/         # Core domain library
└── complish-cli/     # Command-line interface
```

### Key Components

- **Domain Models**: Rich entities with business logic (`Task`, `List`, `Note`, `WorkLog`, etc.)
- **Persistence**: SQLite database for local storage
- **CLI Interface**: Built with `clap` for command parsing and `demand` for interactive prompts

## Development Guidelines

### Code Style

We use standard Rust formatting and linting:

```bash
# Format code and run lints
mise run lint:fix
# Or without mise: ./bin/lint/fix/_default

# Run all checks
mise run lint
# Or without mise: ./bin/lint/_default
```

Our clippy configuration is quite strict (see `Cargo.toml` workspace lints). Key guidelines:

- **No inline comments** - Code should be self-documenting
- Prefer explicit return types over inference where it aids readability
- Use `eyre::Result` for error handling
- Follow the existing patterns for getters/setters using `getset`

### Testing

- **Unit tests** are required for all business logic
- **Integration tests** for CLI commands are encouraged
- Use `pretty_assertions` for test assertions
- Test both success and error cases
- Follow the existing test module organization pattern

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_task() -> Task {
        Task::new(1, "test task")
    }

    mod method_name {
        use super::*;

        #[test]
        fn it_does_the_expected_thing() {
            // Test implementation
        }

        #[test] 
        fn it_handles_error_case() {
            // Error case testing
        }
    }
}
```

### Domain Design Principles

- **Rich domain models**: Business logic belongs in entities, not services
- **Explicit state transitions**: Use methods like `complete()`, `block()`, `cancel()`
- **Immutable by default**: Use `&mut self` only when state actually changes
- **Fail fast**: Validate preconditions and return meaningful errors

### Git Workflow

1. Create a feature branch

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes

    - Write tests first when possible
    - Ensure all tests pass
    - Run formatting and linting

3. Commit with descriptive messages

   ```bash
   git commit -m "Add ability to set task priorities
   
   - Implement Priority enum with p0-p4 levels
   - Add priority field to Task entity
   - Update CLI to accept priority flags"
   ```

4. Push and create a pull request

   ```bash
   git push origin feature/your-feature-name
   ```

## Types of Contributions

### 🐛 Bug Reports

When reporting bugs, please include:

- Steps to reproduce
- Expected vs actual behavior
- Rust version and OS
- Relevant CLI commands and output

### ✨ Feature Requests

For new features:

- Describe the use case and problem being solved
- Consider how it fits with the CLI-first, local-first philosophy
- Check if it aligns with the MVP scope (see README.md)

### 🔧 Code Contributions

**Good first issues:**

- Additional CLI commands for existing functionality
- Improved error messages and help text
- Additional task field validations
- Performance optimizations

**Larger contributions:**

- New integrations (GitHub, Jira, etc.)
- Advanced querying and filtering
- Summary generation improvements
- Database query optimizations

## Architecture Decisions

### Current State (MVP)

- **Local-first**: All data stored locally in user's filesystem
- **CLI-only**: No web interface in MVP
- **Three-list system**: Today/Next/Someday organization
- **Rich domain model**: Business logic in entities, not services
- **SQLite persistence**: Local database storage for tasks and relationships

### Future Considerations

- **Cloud sync**: Planned for post-MVP
- **Desktop app**: Planned for post-MVP
- **Web interface**: Planned for post-MVP
- **Team features**: Planned for post-MVP

When contributing, consider:

- Does this fit the local-first philosophy?
- Is it CLI-appropriate (not requiring GUI)?
- Does it maintain data ownership by the user?
- Can it work offline?

## Getting Help

- **Questions**: Open a GitHub issue with the "question" label
- **Discussions**: Use GitHub Discussions for broader topics
- **Architecture**: Check existing documentation and feel free to ask

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Remember we're all here to build something useful together

## Release Process

Currently using semantic versioning with alpha releases:

- `0.0.1-alpha.0.x.y` format
- Manual releases by maintainers
- Automated cross-compilation for distribution (planned)

## Recognition

Contributors will be acknowledged in release notes. Significant contributions may be recognized with commit access.

---

Thank you for contributing to Complish! Your efforts help make personal productivity tools better for everyone.
