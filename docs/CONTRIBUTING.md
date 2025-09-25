# Contributing to Complish

Thank you for your interest in contributing to Complish! This project is in early development, so these guidelines will
evolve as the project matures.

## Quick Start

### Prerequisites

- [Mise](https://mise.jdx.dev)
- Git

### Setup

1. Fork and clone the repository

    ```sh
    git clone https://github.com/your-username/complish-dev.git
    cd complish-dev
    ```

2. Install dependencies and setup the development environment

    ```sh
    mise run setup
    ```

3. Verify everything works

    ```sh
    mise run ci
    ```

## Development Workflow

### Code Style

We use strict linting and formatting:

```sh
mise run lint:fix  # Format and fix issues
mise run lint      # Check for issues
```

## Getting Help

- **Questions**: Open a GitHub issue
- **Discussions**: Use GitHub Discussions
- **Code of Conduct**: See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

---

> [!NOTE]
> This project is in active development. Contribution guidelines will expand as we approach the first release.
