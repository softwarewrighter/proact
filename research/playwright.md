# Playwright MCP Server Documentation

## Overview

Playwright MCP (Model Context Protocol) is a server that provides browser automation capabilities to AI coding assistants like Claude Code, Gemini CLI, and OpenCode. It enables LLMs to interact with web pages through structured accessibility snapshots, bypassing the need for screenshots or visually-tuned models.

### Key Benefits
- **Accessibility-tree based**: Uses Playwright's accessibility tree instead of pixel-based input, making it more reliable and faster than screenshot-based approaches
- **Deterministic execution**: Avoids ambiguity common with screenshot-based approaches
- **No vision models required**: Operates purely on structured data
- **Cross-platform support**: Works with multiple AI coding assistants through standardized MCP protocol

## General Architecture

The Playwright MCP server implements several architectural principles:

1. **Structured Data Approach**: Instead of relying on visual models or screenshots, the server uses Playwright's accessibility tree to provide structured representations of web pages
2. **Stateful Sessions**: Maintains browser state across interactions, supporting complex multi-step workflows
3. **Tool-Based Interface**: Exposes browser capabilities as discrete tools that AI assistants can invoke
4. **Standardized Protocol**: Uses MCP (Model Context Protocol) for consistent integration across different AI platforms

### Available Tools
- **Navigation**: Navigate to URLs, go back/forward in history
- **Interaction**: Click elements, type text, select options, drag and drop
- **State Management**: Handle dialogs, evaluate JavaScript, manage tabs
- **Inspection**: Take screenshots, capture accessibility snapshots, view console messages
- **Waiting**: Wait for specific text or conditions

### Tool Naming Convention
All Playwright MCP tools follow a consistent naming pattern:
- Claude Code: `mcp__playwright__browser_[action]`
- Other clients may use different prefixes based on their MCP implementation

## Claude Code Configuration

### Installation

There are two main Playwright MCP servers available:

#### Option 1: Official Microsoft Playwright MCP
```bash
claude mcp add playwright -- npx -y @playwright/mcp
```

#### Option 2: ExecuteAutomation Version (Recommended)
```bash
claude mcp add playwright -s user -- npx -y @executeautomation/playwright-mcp-server
```

The `-s user` option specifies the scope as 'user', meaning this server will be available across all your projects.

### Important Notes
- The `claude mcp add` command persists but only affects the directory where you run it
- The MCP server is automatically started by Claude Code when needed
- No separate daemon or service needs to be manually started
- Configuration is stored in `~/.claude.json` with a "projects" key for each directory

### Configuration Options

The Playwright MCP server supports various arguments:
- `--headless`: Run browser in headless mode
- `--isolated`: Keep browser profile in memory
- `--allowed-origins`: Semicolon-separated list of allowed origins
- `--user-data-dir`: Override profile location

### Verification

After setup, verify the installation:
```bash
claude
> /mcp
```
You should see your Playwright server listed as connected.

### Usage in Claude Code

With the MCP loaded, you can:
1. Run `/mcp` and navigate to playwright to view all available tools
2. Use natural language to request browser automation tasks
3. Authentication is easy: have Claude show you a login page, then login yourself with your credentials

### Example Claude Code Commands

```javascript
// Navigate to a webpage
await mcp__playwright__browser_navigate({
    url: "http://localhost:3000"
});

// Click an element
await mcp__playwright__browser_click({
    element: "Submit button",
    ref: "[data-testid='submit-button']"
});

// Type text
await mcp__playwright__browser_type({
    element: "Username input field",
    ref: "#username",
    text: "testuser"
});

// Take a screenshot
await mcp__playwright__browser_take_screenshot({
    fullPage: true
});
```

## Gemini CLI Configuration

### Prerequisites
- Node.js version 20 or higher installed
- Google account for authentication (provides up to 60 requests/minute, 1,000 requests/day)

### Installation

1. Install Gemini CLI following official documentation
2. Configure Playwright MCP:
```bash
npx @playwright/mcp@latest
```

### Architecture Benefits
- Uses a reason and act (ReAct) loop with built-in tools and MCP servers
- Supports both local and remote MCP servers
- Zero cost alternative to Claude Code with similar capabilities
- MCP servers transform the terminal into a fully-fledged IDE with AI integration

### Authentication
When prompted, sign in with your personal Google account to access Gemini API quotas.

### Gemini-Specific Features
- Cost-effective: "Gemini CLI now offers a Claude Code alternative experience at zero cost"
- Performance: "MCP servers work like add-ons that transform the terminal code editor into a fully fledged IDE with AI integration"

## OpenCode Configuration

### Architecture Overview
OpenCode implements MCP to extend capabilities through external tools while being provider-agnostic, supporting over 75 LLM providers.

### Key Features
- **LSP Integration**: Automatically detects programming languages and frameworks, spinning up appropriate Language Server Protocol servers
- **Provider Agnostic**: Works with Claude, GPT-4, and many other models
- **Better Connection Handling**: More robust handling of unstable MCP connections

### Configuration Format

OpenCode uses a JSON configuration structure:

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

### Alternative Configuration (Local MCP)
```json
{
  "mcp": {
    "playwright": {
      "type": "local",
      "command": ["npx", "@playwright/mcp"],
      "enabled": true
    }
  }
}
```

### Benefits
- Configuration is version-controllable
- MCP tools are automatically available alongside built-in tools
- Real-time, accurate code structure mapping reduces hallucinations

## macOS Specific Setup

### Prerequisites for macOS
- macOS 10.15 (Catalina) or later
- Homebrew package manager
- Xcode Command Line Tools: `xcode-select --install`

### Complete macOS Installation

#### 1. Install Node.js and npm
```bash
# Install Node.js via Homebrew (recommended)
brew install node

# Verify installation
node --version
npm --version
```

#### 2. Install Chromium Browser
```bash
# Install Chromium via Homebrew
brew install chromium

# Alternative: Install Google Chrome if preferred
# brew install --cask google-chrome
```

#### 3. Install Playwright Browsers
```bash
# Install Playwright browsers
npx playwright install chromium

# Install all browsers (optional)
npx playwright install
```

### macOS-Specific Considerations

#### Permissions and Security
macOS may require additional permissions for browser automation:

1. **Allow Terminal/Claude Code in System Preferences:**
   - Go to System Preferences → Security & Privacy → Privacy
   - Add Terminal and Claude Code to "Accessibility" if prompted

2. **Browser Security Warnings:**
   - First run may show security warnings
   - Allow Chromium to run in System Preferences → Security & Privacy

#### Path Configuration
Ensure proper PATH configuration in your shell profile (`~/.zshrc` or `~/.bash_profile`):

```bash
# Add npm global packages to PATH
export PATH="$PATH:$(npm prefix -g)/bin"

# Add Homebrew to PATH (if not already added)
export PATH="/opt/homebrew/bin:$PATH"  # For Apple Silicon Macs
# or
export PATH="/usr/local/bin:$PATH"     # For Intel Macs
```

## Best Practices and Troubleshooting

### General Best Practices
1. **Authentication**: Use visible browser windows for manual authentication when needed
2. **State Management**: Be aware that browser state persists across interactions unless using `--isolated` mode
3. **Error Handling**: The accessibility-tree approach provides more reliable element selection than visual methods
4. **Use Test-Friendly Selectors**: Add `data-testid` attributes to HTML elements for reliable testing

### Common Issues and Solutions

#### Connection Issues
- Ensure Node.js version 20+ is installed
- Check that the MCP server command path is correct
- Verify network permissions for browser automation

#### Browser Not Found
- Run the installation command to ensure Playwright browsers are installed
- For Claude Code: The server will prompt to install browsers if missing

#### Browser Version Compatibility
If you encounter browser version mismatches:
```bash
# Example: Link newer version to expected version
ln -sf ~/.cache/ms-playwright/chromium-1181 ~/.cache/ms-playwright/chromium-1179
```

#### macOS Permission Issues
```bash
# Fix npm permissions (if needed)
sudo chown -R $(whoami) $(npm prefix -g)

# Fix Homebrew ownership
sudo chown -R $(whoami) /opt/homebrew/  # Apple Silicon
# or
sudo chown -R $(whoami) /usr/local/     # Intel
```

### Performance Optimization
- Use `--headless` mode for faster execution when visual feedback isn't needed
- Consider `--isolated` mode for testing scenarios to avoid state persistence
- Close unused browser instances to reduce resource usage

### Debugging Commands

```bash
# Check MCP server status (Claude Code)
claude mcp list
claude mcp get playwright

# Test browser installation
npx playwright install --dry-run

# Check system architecture (macOS)
uname -m

# Test MCP server directly
npx @playwright/mcp --help
```

## Additional MCP Clients

Other notable MCP clients that support Playwright:
- **Cursor AI**: With SSE protocol support
- **Windsurf**: Similar development experience to Cursor
- **Continue**: Open-source IDE extension
- **LibreChat**: Docker-based open-source alternative
- **MBro**: Interactive terminal MCP browser client with tab completion

All follow similar configuration patterns with minor syntax variations.

## Security Considerations

- Only use with trusted websites and applications
- Be cautious when executing arbitrary JavaScript
- Consider running in headless mode for production automation
- Validate and sanitize any data extracted from web pages
- Never store credentials in configuration files

## Integration with Development Workflow

Playwright MCP is particularly useful for:
- **Automated Testing**: Test user flows and functionality
- **Data Extraction**: Scrape information from web pages  
- **UI Validation**: Verify visual elements and layouts
- **Performance Testing**: Measure load times and interactions
- **Cross-browser Testing**: Test on different browser engines

## Additional Resources

- [Playwright Documentation](https://playwright.dev/)
- [Microsoft Playwright MCP](https://github.com/microsoft/playwright-mcp)
- [ExecuteAutomation Playwright MCP](https://github.com/executeautomation/playwright-mcp-server)
- [Claude Code MCP Guide](https://docs.anthropic.com/en/docs/claude-code/mcp)
- [Gemini CLI Documentation](https://github.com/google-gemini/gemini-cli)
- [OpenCode Documentation](https://github.com/opencode/opencode)