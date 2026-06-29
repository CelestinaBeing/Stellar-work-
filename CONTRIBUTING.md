# Contributing to StellarWork

Thanks for contributing.

New contributors: start with the [onboarding checklist](docs/contributor-onboarding-checklist.md) and the [docs index](docs/README.md).

## Branching

- Fork and create branches as `feature/<issue-number>-<short-description>`.

## Development Rules

- Contract changes must include or update unit tests.
- Frontend PRs must not break existing pages.
- Use Tailwind utilities only. Do not introduce external UI component libraries.
- Keep scope focused on the linked issue.

## Before Opening a PR

- Run `soroban contract build` in `contracts/escrow`.
- Run `cargo test` in `contracts/escrow`.
- Run `cargo fmt --all -- --check` in `contracts/escrow` to verify formatting.
- Run `make coverage-contract` to verify test coverage meets the 80% threshold.
- Run frontend checks for changed frontend files.

## Contract Code Coverage

To maintain high code quality, we track test coverage for our Soroban smart contracts. We require a minimum of **80% code coverage** for all Rust contract code.

### Running Coverage Locally

You can generate code coverage reports locally using `cargo-tarpaulin`.

1. Ensure you have `cargo-tarpaulin` installed:
   ```bash
   cargo install cargo-tarpaulin
   ```
2. Run the coverage script using the Makefile:
   ```bash
   make coverage-contract
   ```
   Or run the script directly:
   ```bash
   ./contracts/coverage.sh
   ```

This will run the test suite, analyze the coverage, and generate:
- An interactive HTML report at `coverage/tarpaulin-report.html` (open this in your browser to inspect line-by-line coverage).
- An LCOV report at `coverage/lcov.info` (used for CI integration).

## Pre-commit Hooks (Optional)

To ensure consistent code quality, you can optionally set up pre-commit hooks. This will automatically run linting and formatting checks before each commit.

### Setup

1. Install [pre-commit](https://pre-commit.com/#install).
2. Install the hooks in this repository:
   ```bash
   pre-commit install
   ```

### Hook Checks

The following checks are performed:
- **General**: Trailing whitespace, end-of-file fixers, YAML validation, large file check.
- **Rust**: `cargo fmt` (formatting) and `cargo clippy` (linting).
- **Frontend**: `npm run lint` (ESLint).

### Usage and Opt-out

- **Manual Run**: You can run all hooks manually on all files:
  ```bash
  pre-commit run --all-files
  ```
- **Skipping Hooks**: If you need to commit without running hooks (e.g., for a work-in-progress commit), use the `--no-verify` flag:
  ```bash
  git commit -m "your message" --no-verify
  ```
- **Uninstalling**: To remove the hooks:
  ```bash
  pre-commit uninstall
  ```

## Pull Request Requirements

- Reference the issue number in the PR description.
- Include a brief explanation of design choices and trade-offs.
- Include screenshots or short clips for UI changes.
- Maintainer review is required before merge.

## Issue Labels

We use labels to categorize issues and pull requests. Please use them accordingly. For a detailed breakdown of our triage process and label meanings, see the [Issue Triage Guide](docs/TRIAGE.md).

| Label | Description | Example |
| :--- | :--- | :--- |
| `bug` | Something isn't working as expected. | [Bug]: Contract revert on init |
| `enhancement` | New feature or request for improvement. | [Feature]: Add dark mode |
| `documentation` | Improvements or additions to docs. | Add labels guide |
| `good first issue` | Good for newcomers to the project. | Fix typo in README |
| `contract` | Related to Soroban smart contracts. | Update escrow logic |
| `frontend` | Related to the Next.js web application. | Fix navigation alignment |
| `maintenance` | Chore, refactoring, or dependency updates. | Update next.js to latest |
| `invalid` | This doesn't seem right or is out of scope. | Feature request for unrelated app |

## Stale Issue Policy

To keep our issue tracker manageable, we use automated stale issue management:

### Timing
- Issues are marked as **stale** after 30 days of inactivity
- Stale issues are **closed** after 7 additional days (37 days total from last activity)

### Exempt Labels
The following labels exempt issues from being marked stale:
- `bug` - Bugs may need extended investigation
- `security` - Security issues require careful handling
- `good first issue` - Reserved for newcomers
- `help wanted` - Issues seeking community contribution
- `pinned` - Important issues kept visible
- `in-progress` - Currently being worked on
- `blocked` - Waiting on dependencies or external factors

### Removing Stale Status
If an issue is marked as stale, you can remove the stale status by:
- Leaving a comment on the issue
- Updating the issue with new information
- Closing the issue if it's no longer relevant

This resets the inactivity timer.
