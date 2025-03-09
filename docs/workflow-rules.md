# Cursor Workflow Rules

This project has been updated to use the auto rule generator from [cursor-auto-rules-agile-workflow](https://github.com/bmadcode/cursor-auto-rules-agile-workflow).

> **Note**: This script can be safely re-run at any time to update the template rules to their latest versions. It will not impact or overwrite any custom rules you've created.

## Core Features

- Automated rule generation
- Standardized documentation formats
- AI behavior control and optimization
- Flexible workflow integration options

## Workflow Integration Options

### 1. Automatic Rule Application (Recommended)
The core workflow rules are automatically installed in `.cursor/rules/`:
- `901-prd.mdc` - Product Requirements Document standards
- `902-arch.mdc` - Architecture documentation standards
- `903-story.mdc` - User story standards
- `801-workflow-agile.mdc` - Complete Agile workflow (optional)

These rules are automatically applied when working with corresponding file types.

### 2. Notepad-Based Workflow
For a more flexible approach, use the templates in `xnotes/`:
1. Enable Notepads in Cursor options
2. Create a new notepad (e.g., "agile")
3. Copy contents from `xnotes/workflow-agile.md`
4. Use \`@notepad-name\` in conversations

> 💡 **Tip:** The Notepad approach is ideal for:
> - Initial project setup
> - Story implementation
> - Focused development sessions
> - Reducing context overhead

## Getting Started

1. Review the templates in \`xnotes/\`
2. Choose your preferred workflow approach
3. Start using the AI with confidence!

For demos and tutorials, visit: [BMad Code Videos](https://youtube.com/bmadcode)
