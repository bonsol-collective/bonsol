---
description: ALWAYS use when making commits or managing git workflow to ensure consistent version control practices. This rule enforces conventional commit messages and proper staging/push procedures.
globs: 
---
<rule>
- Run the command `git add .` from the root of the workspace
- Review all added changes that will be included in the commit
- Create a git commit message without newline characters
- Use format: `type(scope): brief description`
- Keep titles brief and descriptive (max 72 chars)
- Add two new lines after commit title without newline characters
- Include diff summary of all changes
- Add detailed explanations in commit body
- End with signature "-bmadAi"
- Push all to the remote of whatever the current branch is
</rule>

<example>
type(scope): brief description

Changes made in this commit:
- Modified files: [list of modified files]
- Added files: [list of added files]
- Deleted files: [list of deleted files]
- Key changes:
  - [specific change 1]
  - [specific change 2]
  ...

Detailed explanation of changes and reasoning...

-bmadAi
</example>

<example type="invalid">
fixed stuff
</example> 