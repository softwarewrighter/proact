
I want to use Claude Code (CLI) with Playwright MCP server.  How do I add it using claude mcp add?

Edit

Investigated Claude Code usage and recommended seeking documentation.
6s






I don't have specific documentation about the claude mcp add command or detailed instructions for setting up MCP servers with Claude Code. Since Claude Code is in research preview, the specific commands and setup procedures would be found in the official documentation.

For detailed information about Claude Code commands and MCP server integration, I'd recommend checking:

Anthropic's blog - where Claude Code was announced and initial documentation is available
Anthropic's documentation site at https://docs.anthropic.com - which may have updated Claude Code documentation
The Claude Code repository or documentation - if it's been published with setup instructions
If you can't find the specific claude mcp add syntax there, you might also try running claude --help or claude mcp --help in your terminal to see the available commands and their usage.

Since you're interested in architecture and design considerations, you'll probably also want to understand how the MCP (Model Context Protocol) server architecture works with Claude Code for your Playwright integration - that context will likely be in the official documentation as well.




Retry
I'm trying to understand how Playwright (as an MCP server?) works and what the claude mcp add command should look like (and what process has to be manually started, if any).

Edit

Investigating Playwright's MCP server and Claude command details
3s





I'll help you understand how Playwright works as an MCP server and the proper claude mcp add command. Let me search for the current documentation and setup procedures.


Claude Code MCP add Playwright server setup
10 results






















Great! Now I have comprehensive information about how to set up Playwright as an MCP server with
0s



Claude Code MCP documentation command reference
10 results






















Decoded Playwright MCP server setup and command syntax.
6s




Based on my research, here's how Playwright works as an MCP server and the proper setup for Claude Code:

How Playwright MCP Works
Playwright MCP is "A Model Context Protocol (MCP 
GitHub - microsoft/playwright-mcp: Playwright MCP server
) server that provides browser automation capabilities using Playwright. This server enables LLMs to interact with web pages through structured accessibility snapshots, bypassing the need for screenshots or visually-tuned models."

Architecture & Design Rationale
The Playwright MCP server uses several key architectural principles:

Accessibility-tree based: Uses Playwright's accessibility tree, not pixel-based input 
GitHub - microsoft/playwright-mcp: Playwright MCP server
, making it more reliable and faster than screenshot-based approaches
Deterministic execution: Avoids ambiguity common with screenshot-based approaches 
GitHub - microsoft/playwright-mcp: Playwright MCP server
No vision models required: operates purely on structured data 
GitHub - microsoft/playwright-mcp: Playwright MCP server
Claude MCP Add Command
There are two main Playwright MCP servers you can use:

Option 1: Official Microsoft Playwright MCP
bash
claude mcp add playwright -- npx -y @playwright/mcp
Option 2: ExecuteAutomation Version (Recommended)
bash
claude mcp add playwright -s user -- npx -y @executeautomation/playwright-mcp-server
The -s user option "Specifies the scope as 'user', meaning this server will be available across all your projects" 
How to Install MCP Servers in Claude Code
, which aligns with your preference for maintainability.

No Manual Process Required
Important: The claude mcp add command will persist but will only affect the directory in which you run it 
Using Playwright MCP with Claude Code | Simon Willison’s TILs
, and the MCP server is automatically started by Claude Code when needed. No separate daemon or service needs to be manually started.

Addressing Your "-ilities" Requirements
Reliability & Availability: The server runs in either persistent profile mode (like a regular browser) or isolated contexts for testing sessions 
GitHub - microsoft/playwright-mcp: Playwright MCP server

Maintainability: Configuration gets stored in ~/.claude.json which includes a "projects" key with objects for each directory 
Using Playwright MCP with Claude Code | Simon Willison’s TILs

Discoverability: With the MCP loaded you can run /mcp and then navigate to playwright to view all available tools 
Using Playwright MCP with Claude Code | Simon Willison’s TILs

Usability: Since Claude uses a visible browser window when interacting with Playwright, authentication is easy: have it show you a login page, then login yourself with your own credentials 
Using Playwright MCP with Claude Code | Simon Willison’s TILs

Configuration Options
The Playwright MCP server supports various arguments for customization:

--headless: Run browser in headless mode
--isolated: Keep browser profile in memory
--allowed-origins: Semicolon-separated list of allowed origins
--user-data-dir: Override profile location
Verification
After setup, verify with:

bash
claude
> /mcp
You should see your Playwright server listed as connected.

This setup provides the architectural foundation for reliable browser automation while maintaining the reproducibility and literate programming approaches you prefer.




Retry
Is there similar documentation for using Playwright (as an MCP server) with Gemini CLI or opencode (Claude Code CLI alternatives)?

Edit

Investigated MCP server compatibility with alternative CLI tools.
2s




Let me search for documentation on using Playwright MCP with Gemini CLI and opencode as alternatives to Claude Code.


Gemini CLI MCP server Playwright support
10 results






















Now let me search for information about "opencode" as an alternative CLI tool and its MCP support.
0s



opencode CLI MCP server support Playwright alternative Claude Code
10 results






















Great! Now I have comprehensive information about both Gemini CLI and OpenCode's MCP support.
1s



"MCP clients" alternatives "Claude Code" terminal AI coding
8 results


















Synthesized MCP client alternatives for Playwright documentation.
1s



Based on my research, there is similar documentation for using Playwright MCP with both Gemini CLI and OpenCode as alternatives to Claude Code. Here's a comprehensive overview:

Gemini CLI with Playwright MCP
Architecture & Support
The official Google Gemini CLI "uses a reason and act (ReAct) loop with your built-in tools and local or remote MCP servers to complete complex use cases like fixing bugs, creating new features, and improving test coverage." 
GitHub - google-gemini/gemini-cli: An open-source AI agent that brings the power of Gemini directly into your terminal.

Gemini CLI supports "MCP servers to connect new capabilities" 
GitHub - google-gemini/gemini-cli: An open-source AI agent that brings the power of Gemini directly into your terminal.
 and has built-in integration for the Model Context Protocol.

Setup with Playwright MCP
The setup process mirrors Claude Code but uses Gemini CLI's configuration:

Install Gemini CLI: Prerequisites: "Ensure you have Node.js version 20 or higher installed" 
GitHub - google-gemini/gemini-cli: An open-source AI agent that brings the power of Gemini directly into your terminal.
Configure Playwright MCP: You can use the standard Playwright MCP server:
bash
# Similar to Claude Code setup
npx @playwright/mcp@latest
Authentication: "When prompted, sign in with your personal Google account. This will grant you up to 60 model requests per minute and 1,000 model requests per day using Gemini." 
GitHub - google-gemini/gemini-cli: An open-source AI agent that brings the power of Gemini directly into your terminal.
Advantages for Your Use Case
Cost-effective: "Gemini CLI now offers a Claude Code alternative experience at zero cost" 
How I‘m Using Gemini CLI + MCP Servers To Level Up to Claude Code (FREE Effective Alternative) | by Joe Njenga | Jun, 2025 | Medium
Performance: "MCP servers work like add-ons that transform the terminal code editor into a fully fledged IDE with AI integration" 
These 6 Gemini CLI MCP Servers (Are Making It Go Beast Mode!) | by Joe Njenga | Jul, 2025 | Medium
OpenCode with Playwright MCP
Architecture Overview
OpenCode "implements the Model Context Protocol (MCP) to extend its capabilities through external tools. MCP provides a standardized way for the AI assistant to interact with external services and tools." 
Top 7 MCP Clients for AI Tooling - KDnuggets

Key Architectural Benefits
Provider Agnostic: "While OpenCode works great with Claude, its architecture is provider-agnostic. It supports over 75 LLM providers" 
How to Install Playwright MCP in Claude Code? · Issue #534 · microsoft/playwright-mcp
LSP Integration: "When it launches, it inspects your project directory and automatically detects the programming language and frameworks you're using. It then silently spins up the appropriate Language Server Protocol (LSP) server" 
How to Install Playwright MCP in Claude Code? · Issue #534 · microsoft/playwright-mcp
Configuration Format
OpenCode uses a different configuration structure in its config file: 
Top 7 MCP Clients for AI Tooling - KDnuggets

json
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
Addressing Your "-ilities" Requirements
Reliability: OpenCode handles unstable MCP connections better than some alternatives, though there are workarounds for connection issues 
GitHub - wong2/awesome-mcp-servers: A curated list of Model Context Protocol (MCP) servers

Maintainability: The configuration is stored in a standard JSON format that's version-controllable

Discoverability: "Once configured, MCP tools are automatically available to the AI assistant alongside built-in tools" 
Top 7 MCP Clients for AI Tooling - KDnuggets

Other Notable MCP Clients
Additional Alternatives
Several other terminal-based MCP clients exist, including: 
How to Quickly Build a MCP Server for Claude Code

Cursor AI: With SSE protocol support
Windsurf: Similar development experience to Cursor
Continue: Open-source IDE extension
LibreChat: Docker-based open-source alternative
MBro: "A powerful interactive terminal MCP Browser client with tab completion and automatic documentation"
Universal Compatibility
The Playwright MCP server documentation shows it works with multiple clients: "Go to Cursor Settings -> MCP -> Add new MCP Server... Follow Windsurf MCP documentation. Use the standard config above." 
MCP Gemini CLI | MCP Servers · LobeHub

Configuration Examples
For Gemini CLI
The configuration typically involves setting up the MCP server and then running Gemini CLI in your project directory, similar to Claude Code.

For OpenCode
OpenCode supports both local and remote MCP configurations: 
GitHub - wong2/awesome-mcp-servers: A curated list of Model Context Protocol (MCP) servers

json
{
  "mcp": {
    "playwright": {
      "type": "local",
      "command": ["npx", "@playwright/mcp"],
      "enabled": true
    }
  }
}
Architectural Considerations
Performance: OpenCode's LSP integration means "the LLM is far less likely to hallucinate non-existent functions or misuse APIs because it has a real-time, accurate map of your code's structure" 
How to Install Playwright MCP in Claude Code? · Issue #534 · microsoft/playwright-mcp

Extensibility: Both tools follow MCP standards, ensuring "External Tool Integration: Connect to external tools and services via a standardized protocol" 
Top 7 MCP Clients for AI Tooling - KDnuggets

Reproducibility: All configurations are file-based and can be version controlled, supporting your literate programming and reproducible research preferences.

The documentation and setup procedures are quite similar across these tools, with the main differences being in configuration file formats and authentication methods. The underlying Playwright MCP server functionality remains consistent across all clients.




Retry





