---
icon: hands-holding-heart
---

# Contributor guidelines

Thank you for your interest in contributing to Bonsol! This guide will help you understand our contribution workflow and requirements.

## Local Setup

Refer to [setup-a-local-environment.md](../developers/setup-a-local-environment.md "mention")to get started with a local environment for contributing. This will allow you to build, run and make changes to Bonsol from source.&#x20;

## Pull Requests

When submitting pull requests to Bonsol, please follow these guidelines:

1. **Check Existing Issues**: Before creating a new issue or PR, verify if an existing issue already addresses your concern.
2. **Issue References**: All PRs should reference a corresponding GitHub issue using closing keywords (example: `Closes #123`).
3. **Clear Descriptions**: Include a clear, concise description of your changes in the PR.
4. **Testing**: Add relevant tests that demonstrate your changes work as intended.
5. **Code Quality**:
   * Run `nix flake check` locally to verify your changes pass our quality checks
   * Format Rust code with `cargo +nightly fmt`
   * Format TOML files with `taplo fmt`
   * Check for lints with `cargo clippy`
6. **Dependencies**: Exercise caution when adding new dependencies to ensure they're well-maintained and secure.

## Commit Messages <a href="#commit-message-guidelines" id="commit-message-guidelines"></a>

We use **commitlint** to ensure that all commit messages follow a consistent style based on the [Conventional Commits](https://www.conventionalcommits.org) specification. This makes it easier to understand the history of the project and generate changelogs automatically.

#### Commit Message Format <a href="#commit-message-format" id="commit-message-format"></a>

Each commit message must be structured as follows:

**Type**

The type must be one of the following:

* **feat**: A new feature
* **fix**: A bug fix
* **docs**: Documentation only changes
* **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc.)
* **refactor**: A code change that neither fixes a bug nor adds a feature
* **perf**: A code change that improves performance
* **test**: Adding missing or correcting existing tests
* **build**: Changes that affect the build system or external dependencies (example scopes: gulp, npm)
* **ci**: Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)
* **chore**: Other changes that don't modify src or test files
* **revert**: Reverts a previous commit

**Scope**

The scope is optional and provides additional context about what the commit affects (e.g., `api`, `cli`, `frontend`, etc.).

**Description**

The description is a short, imperative summary of the change. It should start with a verb and be written in the present tense (e.g., "add feature," "fix bug").

**Body (optional)**

The body of the commit message provides additional details about the change. Use this when the change is not trivial and requires more explanation.

**Footer (optional)**

The footer should contain any relevant information about breaking changes or issues being closed:

* Breaking changes should start with the word `BREAKING CHANGE:`, followed by an explanation of what changed and why.
* Issues should be referenced using the `Closes` keyword, like so: `Closes #123`.

#### Example Commit Messages <a href="#example-commit-messages" id="example-commit-messages"></a>

```
feat(api): add user authentication
```

```
fix(auth): correct token expiration logic
```

```
docs: update README with new installation steps
```

```
chore: update dependencies
```
