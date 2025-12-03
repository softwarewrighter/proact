## 4. Playwright MCP Server Setup and Usage

### Overview

Playwright MCP (Model Context Protocol) is a server that provides browser automation capabilities to AI coding assistants. It enables interaction with web pages through structured accessibility snapshots, bypassing the need for screenshots or visually-tuned models.

### Key Benefits
- **Accessibility-tree based**: Uses Playwright's accessibility tree instead of pixel-based input
- **Deterministic execution**: Avoids ambiguity common with screenshot-based approaches
- **No vision models required**: Operates purely on structured data
- **Cross-platform support**: Works with multiple AI coding assistants through standardized MCP protocol

### Installation Instructions

#### For Claude Code
```bash
# User-level installation (recommended - available across all projects)
claude mcp add playwright -s user -- npx -y @playwright/mcp

# Verify installation
claude mcp list
```

#### For Gemini CLI
```bash
# Ensure Node.js v20+ is installed
# Configure Playwright MCP server
npx @playwright/mcp@latest
```

#### For OpenCode
Configure in the MCP settings file:
```json
{
  "mcpServers": {
    "playwright": {
      "type": "stdio",
      "command": "npx",
      "args": ["@playwright/mcp"],
      "env": []
    }
  }
}
```

### Tool Naming Convention
All Playwright MCP tools follow a consistent naming pattern:
- Claude Code: `mcp__playwright__[action]`
- Other clients may use different prefixes based on their MCP implementation

### Available Tools
- **Navigation**: Navigate to URLs, go back/forward in history
- **Interaction**: Click elements, fill forms, select options, drag and drop
- **State Management**: Handle dialogs, evaluate JavaScript, manage tabs
- **Inspection**: Take screenshots, capture accessibility snapshots, view console logs
- **Waiting**: Wait for specific elements or conditions

### Best Practices
1. **Use User-Level Setup**: Configure frequently-used MCP servers at user level for consistency
2. **Project-Level for Specific Needs**: Use project-level configuration only for project-specific tools
3. **Regular Updates**: Periodically update the MCP server to get latest features
4. **Consistent Naming**: Always use "playwright" as the server name for consistency

### Troubleshooting

#### Browser Installation
If you encounter browser-related errors:
```bash
# Install Chromium (recommended)
npx playwright install chromium

# Or install all browsers
npx playwright install
```

#### Verification
After setup, verify the connection:
```bash
# For Claude Code
claude
> /mcp

# Should show: playwright: npx -y @playwright/mcp - ✓ Connected
```

---
