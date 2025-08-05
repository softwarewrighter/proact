# Playwright MCP User-Level Setup Guide

## Overview

This guide explains how to set up Playwright MCP server at the user level, making it available across all your projects without needing to configure it for each one individually.

## Benefits of User-Level Setup

- **Consistency**: Same Playwright configuration across all projects
- **Speed**: No need to reconfigure for each new project
- **Simplicity**: One-time setup that works everywhere
- **Maintenance**: Update in one place affects all projects

## Initial Setup (One-Time Only)

### 1. Remove Any Existing Project-Specific Configurations

If you have Playwright MCP configured in specific projects, remove them first:

```bash
# Remove local configuration (private to you in a project)
claude mcp remove playwright -s local

# Remove project configuration (shared via .mcp.json)
claude mcp remove playwright -s project
```

### 2. Install Playwright MCP at User Level

```bash
# Add Playwright MCP server for all projects
claude mcp add playwright -s user -- npx -y @playwright/mcp
```

### 3. Verify Installation

```bash
# Check that Playwright is connected
claude mcp list
```

You should see:
```
playwright: npx -y @playwright/mcp - ✓ Connected
```

## Using Playwright in Any Project

Once configured at the user level, Playwright MCP is automatically available in any project. Simply start using the tools:

### Example Usage

```javascript
// Navigate to a page
await mcp__playwright__browser_navigate({
    url: "https://example.com"
});

// Take a screenshot
await mcp__playwright__browser_take_screenshot({
    fullPage: true
});

// Click an element
await mcp__playwright__browser_click({
    element: "Submit button",
    ref: "button[type='submit']"
});
```

## Quick Reference for New Projects

When starting a new project, no Playwright MCP setup is needed! Just:

1. Navigate to your project directory
2. Start Claude Code: `claude`
3. Use Playwright tools immediately

## Checking Your Configuration

To verify Playwright is configured at user level:

```bash
# Check user-level MCP servers
cat ~/.claude.json | jq '.mcpServers | keys'
```

Should output:
```json
[
  "playwright"
]
```

## Troubleshooting

### If Playwright isn't available in a project

1. Check if there's a conflicting project-specific configuration:
   ```bash
   cat .mcp.json 2>/dev/null || echo "No project config"
   ```

2. Verify user-level configuration exists:
   ```bash
   claude mcp list
   ```

3. If missing, re-add at user level:
   ```bash
   claude mcp add playwright -s user -- npx -y @playwright/mcp
   ```

### Browser Installation Issues

If you get browser-related errors, install Playwright browsers:

```bash
# Install Chromium (recommended)
npx playwright install chromium

# Or install all browsers
npx playwright install
```

## Configuration Management

### View Current Configuration

```bash
# List all MCP servers and their scopes
claude mcp list

# Get details about Playwright configuration
claude mcp get playwright
```

### Update Configuration

To update the Playwright MCP server:

```bash
# Remove old configuration
claude mcp remove playwright -s user

# Add updated configuration
claude mcp add playwright -s user -- npx -y @playwright/mcp@latest
```

## Best Practices

1. **Use User-Level for Common Tools**: Configure frequently-used MCP servers like Playwright at user level
2. **Project-Level for Specific Needs**: Use project-level configuration only for project-specific tools
3. **Regular Updates**: Periodically update the MCP server to get latest features
4. **Consistent Naming**: Always use "playwright" as the server name for consistency

## Advanced Options

### Custom Browser Configuration

You can add additional arguments when setting up:

```bash
# Headless mode by default
claude mcp add playwright -s user -- npx -y @playwright/mcp --headless

# Custom user data directory
claude mcp add playwright -s user -- npx -y @playwright/mcp --user-data-dir=/path/to/profile

# Isolated mode (no persistent state)
claude mcp add playwright -s user -- npx -y @playwright/mcp --isolated
```

### Multiple Browser Support

The default setup uses Chromium, but Playwright supports multiple browsers. The MCP server will use whichever browsers you have installed via `npx playwright install`.

## Summary

With user-level setup:
- ✅ Configure once, use everywhere
- ✅ No per-project setup needed
- ✅ Consistent browser automation across all projects
- ✅ Easy to maintain and update

This approach significantly streamlines your workflow when working with multiple projects that need browser automation capabilities.