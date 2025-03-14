---
icon: book-open
---

# Contributing to Documentation

This guide explains how to contribute to the Bonsol documentation through GitHub. All our documentation is stored in the `gitbook/` directory of our repository and synchronized with GitBook.

## Understanding the Documentation Structure

Our documentation follows this structure:

- `gitbook/README.md` - The main landing page of the documentation
- `gitbook/SUMMARY.md` - Defines the structure and navigation of the documentation
- `gitbook/core-concepts/` - Conceptual information about Bonsol
- `gitbook/getting-started/` - Guides for new users
- `gitbook/developers/` - Resources for developers
- `gitbook/provers/` - Information for provers
- `gitbook/contributing/` - Guidelines for contributors (including this document)

## How to Add or Update Documentation

### 1. Fork and Clone the Repository

1. Fork the Bonsol repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/bonsol.git
   cd bonsol
   ```

### 2. Create a New Branch

Create a new branch for your documentation changes:

```bash
git checkout -b docs/your-documentation-change
```

Use a descriptive name that indicates what you're documenting.

### 3. Making Documentation Changes

#### Adding a New Page

1. Create your new markdown file in the appropriate subdirectory in `gitbook/`
2. Add a reference to your new file in `gitbook/SUMMARY.md` to make it appear in the navigation
   
Example SUMMARY.md addition:
```markdown
## Getting Started

* [Installation](getting-started/installation.md)
* [Quickstart](getting-started/quickstart.md)
* [Your New Page](getting-started/your-new-page.md)
```

#### Updating Existing Content

Simply edit the relevant markdown files in the `gitbook/` directory.

#### Markdown Guidelines

- Use clear headings with proper hierarchy (# for title, ## for sections, etc.)
- Add code examples with proper syntax highlighting:
  ````markdown
  ```rust
  fn main() {
      println!("Hello, Bonsol!");
  }
  ```
  ````
- Use relative links when referencing other documentation pages
- Include screenshots or diagrams when they help explain concepts
- Follow our style guide for consistent documentation

### 4. Preview Your Changes

You can also use standard Markdown previewers to check your content. 

### 5. Create a Pull Request

1. Commit your changes:
   ```bash
   git add gitbook/
   git commit -m "docs: add documentation for X feature"
   ```
   
   Follow our [commit message guidelines](contributor-guidelines.md#commit-message-guidelines) with the type "docs".

2. Push to your fork:
   ```bash
   git push origin docs/your-documentation-change
   ```

3. Create a pull request on GitHub
   - Reference any related issues
   - Provide a clear description of what documentation you've added or updated
   - Request review from relevant team members

## Documentation Style Guide

To maintain consistent documentation:

- Use present tense and active voice
- Be concise but thorough
- Include examples where appropriate
- Use sentence case for headings
- Link to relevant documentation sections
- Keep paragraphs short and focused

## GitBook Specific Features

Our documentation takes advantage of several GitBook features:

### Page Icons

You can add an icon to your page by including this at the top of your markdown file:

```markdown
---
icon: name-of-icon
---
```

### Internal Page References

To reference another page in the documentation:

```markdown
[Title of Page](../path/to/page.md "mention")
```

## Questions?

If you have any questions about contributing to documentation, please open an issue on GitHub or contact the team through our community channels.

Thank you for helping improve Bonsol's documentation! 
