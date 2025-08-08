# Contributing to Complish

Thank you for your interest in contributing to Complish! This project is in early development, so these guidelines will
evolve as the project matures.

## Quick Start

### Prerequisites

- [Mise](https://mise.jdx.dev)
- Git

### Setup

1. Fork and clone the repository

    ```bash
    git clone https://github.com/your-username/complish-dev.git
    cd complish-dev
    ```

2. Install mise and setup the development environment

    ```bash
    mise install
    ```

3. Verify everything works

    ```bash
    mise run ci
    ```

## Development Workflow

### Code Style

We use strict linting and formatting:

```bash
mise run lint:fix  # Format and fix issues
mise run lint      # Check for issues
```

### Testing

- Write tests for business logic
- Use `pretty_assertions` for test assertions
- Follow the existing nested module test pattern

### Git Workflow

1. Create a feature branch: `git checkout -b feature/your-feature-name`
2. Make your changes with tests
3. Ensure `mise run ci` passes
4. Push and create a pull request

## Project Philosophy

Complish is **local-first** and **CLI-first**. When contributing, consider:

- Does this work offline?
- Does this fit a command-line workflow?
- Does this keep user data under their control?

## Getting Help

- **Questions**: Open a GitHub issue
- **Discussions**: Use GitHub Discussions
- **Code of Conduct**: See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

---

> [!NOTE]
> This project is in active development. Contribution guidelines will expand as we approach the first release.
