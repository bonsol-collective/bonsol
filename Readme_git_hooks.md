# Pre-commit Git Hooks for Your Repository



# Pre-Commit Git Hooks

## Purpose

- Automate code style checks.
- Prevent committing broken code.
- Run security and linting checks.
- Ensure code consistency across the team.

## Installation

### Prerequisites

Ensure you have **Git** and **Python** installed. Python version 3.10 is required to be installed for a correct functionality.

### Install `pre-commit`

```sh
pip install pre-commit
```

Alternatively, install using **Homebrew** (macOS/Linux):

```sh
brew install pre-commit

   ```
Verify the installation by running:
   ```bash
 pre-commit run --all-files
```

### Enable Pre-Commit in Your Repository

Run the following command inside your repository to set up `pre-commit`:

```sh
pre-commit install
```

This will configure Git to trigger the hooks before each commit.

## Usage

### Automation

Pre-commit runs automatically when you issue git commands as `git commit` and `git push`. Any issues found have to be corrected before you can commit your changes locally and push them to Github.

### Running Hooks Manually

To manually check your files without committing, run:

```sh
pre-commit run --all-files
```

### Skipping Pre-Commit Checks

If necessary, you can bypass `pre-commit` checks when committing:

```sh
git commit --no-verify
```

> ⚠️ **Use this only when absolutely necessary, as it skips all configured checks.**

### Updating Hooks

To update all installed hooks to their latest versions, run:

```sh
pre-commit autoupdate
```

## Configuration

Pre-commit hooks are configured in the `.pre-commit-config.yaml` Following checks are configured to run:

```yaml
pre-commit:
  - id: check-yaml
  - id: end-of-file-fixer
  - id: trailing-whitespace
  - id: check-added-large-files
  - id: check-case-conflict
  - id: check-toml
  - id: mixed-line-ending
  - id: shellcheck
  - id: talisman-commit
  - id: typos
  - id: dockerfilelint

pre-commit message:
  - id: commitizen

pre-push:
  - id: untracked-files
```
