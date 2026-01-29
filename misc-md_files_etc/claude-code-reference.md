# Claude Code Reference Manual

## CLI Invocation

```bash
claude                      # Interactive session
claude -c                   # Continue most recent session
claude -r                   # Resume picker (search past sessions)
claude -r <session-id>      # Resume specific session
claude -p "prompt"          # Non-interactive, print and exit
claude --model opus         # Specify model (sonnet, opus, haiku)
claude --verbose            # Detailed tool output
```

### Key CLI Flags

| Flag | Purpose |
|------|---------|
| `-p, --print` | Non-interactive mode, outputs response and exits |
| `-c, --continue` | Continue most recent conversation |
| `-r, --resume [id]` | Resume session by ID or open picker |
| `--model <model>` | Set model (sonnet, opus, haiku, or full name) |
| `--system-prompt` | Override system prompt |
| `--append-system-prompt` | Add to default system prompt |
| `--allowed-tools` | Whitelist specific tools |
| `--disallowed-tools` | Blacklist specific tools |
| `--add-dir` | Grant access to additional directories |
| `--verbose` | Show detailed execution |
| `--debug` | Enable debug output |
| `--mcp-config` | Load MCP server configuration |
| `--permission-mode` | Set permission handling (default, plan, bypassPermissions) |

---

## Keyboard Commands

### General Controls

| Shortcut | Action |
|----------|--------|
| `Ctrl+C` | Cancel current operation |
| `Ctrl+D` | Exit session (EOF) |
| `Ctrl+L` | Clear screen (preserves history) |
| `Ctrl+O` | Toggle verbose output |
| `Ctrl+R` | Reverse search command history |
| `Up/Down` | Navigate input history |
| `Esc Esc` | Rewind conversation/code |
| `Shift+Tab` | Cycle permission modes |
| `Alt+M` | Cycle permission modes (alternate) |

### Text Editing

| Shortcut | Action |
|----------|--------|
| `Ctrl+K` | Delete to end of line |
| `Ctrl+U` | Delete entire line |
| `Ctrl+Y` | Paste deleted text |
| `Alt+Y` | Cycle paste history |
| `Alt+B` | Move back one word |
| `Alt+F` | Move forward one word |
| `Ctrl+A` | Move to beginning of line |
| `Ctrl+E` | Move to end of line |

### Session Management

| Shortcut | Action |
|----------|--------|
| `Ctrl+G` | Open prompt in external editor |
| `Ctrl+B` | Background current task |
| `Ctrl+T` | Toggle task list view |
| `Alt+P` / `Option+P` | Switch model |
| `Alt+T` / `Option+T` | Toggle extended thinking |

### Multiline Input

| Method | Shortcut |
|--------|----------|
| Escape newline | `\ + Enter` |
| macOS default | `Option+Enter` |
| iTerm2/WezTerm/Kitty | `Shift+Enter` |
| Line feed | `Ctrl+J` |

---

## Slash Commands

### Session Management

| Command | Purpose |
|---------|---------|
| `/clear` | Clear conversation history |
| `/compact [focus]` | Compress context, optional focus instructions |
| `/resume [session]` | Resume previous session |
| `/exit` | Exit Claude Code |
| `/rename <name>` | Rename current session |

### Information & Diagnostics

| Command | Purpose |
|---------|---------|
| `/help` | Show help |
| `/context` | Visualize context window usage |
| `/cost` | Show token usage and costs |
| `/stats` | Daily usage statistics |
| `/doctor` | Check installation health |
| `/tasks` | List background tasks |
| `/todos` | Show current todo items |
| `/usage` | Show plan limits (subscription) |

### Configuration

| Command | Purpose |
|---------|---------|
| `/config` | Open settings interface |
| `/model` | Change model |
| `/permissions` | View/edit permissions |
| `/theme` | Change color theme |
| `/statusline` | Configure status line |
| `/memory` | Edit CLAUDE.md files |
| `/mcp` | Manage MCP servers |
| `/vim` | Enable vim editing mode |
| `/plan` | Enter plan mode |

### Output

| Command | Purpose |
|---------|---------|
| `/export [file]` | Export conversation (clipboard if no file) |
| `/init` | Create CLAUDE.md project guide |

---

## Input Prefixes

| Prefix | Function | Example |
|--------|----------|---------|
| `/` | Slash command | `/compact` |
| `!` | Direct shell execution | `!git status` |
| `@` | File path autocomplete | `@src/main.rs` |

---

## Operating Modes

### Normal Mode (default)
- Asks permission for file edits, shell commands
- Interactive approval workflow

### Plan Mode (`/plan` or `--permission-mode plan`)
- Read-only exploration
- Produces implementation plan
- Requires explicit approval before execution

### Auto-Accept Mode (`Shift+Tab` to cycle)
- Bypasses permission prompts
- Use in trusted environments only

### Verbose Mode (`Ctrl+O`)
- Shows full tool execution details
- Useful for debugging/understanding behavior

---

## File Operations

### Reading
```
Look at @src/lib.rs
Read /path/to/file.txt
```

### Editing
Claude edits existing files in place. Diffs shown for approval.

### Creating
Only creates files when necessary. Prefers editing existing.

### Images
```
Analyze @/path/to/image.png
[Ctrl+V to paste from clipboard]
```

---

## Context Management

### Context Window
- Conversation has unlimited effective length via automatic summarization
- `/context` shows current token usage
- `/compact` manually compresses with optional focus

### File References
- `@path/to/file` - includes file in context
- Multiple files: `@file1.rs @file2.rs`
- Line references: `@file.rs:50-100`

### CLAUDE.md Files
Project-level memory/instructions. Claude reads automatically:
- `./CLAUDE.md` - project root
- `~/.claude/CLAUDE.md` - global

Edit with `/memory`

---

## Background Operations

### Backgrounding Tasks
```
Ctrl+B          # Background current operation
/tasks          # List all background tasks
```

### Background Shell Commands
Long-running processes (builds, servers) can run in background.
Output captured, retrievable via task ID.

---

## MCP (Model Context Protocol)

External tool servers extending Claude's capabilities.

```bash
claude mcp list                    # Show configured servers
claude mcp add <name> <command>    # Add server
claude mcp remove <name>           # Remove server
```

In-session: `/mcp`

---

## IDE Integration

### VS Code

| Shortcut | Action |
|----------|--------|
| `Cmd+Esc` / `Ctrl+Esc` | Toggle focus to Claude panel |
| `Cmd+Shift+Esc` / `Ctrl+Shift+Esc` | Open in new tab |
| `Cmd+N` / `Ctrl+N` | New conversation |
| `Option+K` / `Alt+K` | Insert file reference with line numbers |

### JetBrains

| Shortcut | Action |
|----------|--------|
| `Cmd+Esc` / `Ctrl+Esc` | Open Claude panel |
| `Cmd+Option+K` / `Alt+Ctrl+K` | Insert file reference |

---

## Vim Mode

Enable: `/vim`

### Mode Switching

| From | Command | To |
|------|---------|-----|
| INSERT | `Esc` | NORMAL |
| NORMAL | `i` | INSERT (before cursor) |
| NORMAL | `I` | INSERT (line start) |
| NORMAL | `a` | INSERT (after cursor) |
| NORMAL | `A` | INSERT (line end) |
| NORMAL | `o` | INSERT (new line below) |
| NORMAL | `O` | INSERT (new line above) |

### Navigation (NORMAL)

| Command | Motion |
|---------|--------|
| `h j k l` | Left, down, up, right |
| `w` | Next word |
| `b` | Previous word |
| `e` | End of word |
| `0` | Line start |
| `$` | Line end |
| `^` | First non-blank |
| `gg` | Document start |
| `G` | Document end |
| `f{c}` | Jump to char |
| `F{c}` | Jump back to char |

### Editing (NORMAL)

| Command | Action |
|---------|--------|
| `x` | Delete char |
| `dd` | Delete line |
| `D` | Delete to EOL |
| `dw` | Delete word |
| `cc` | Change line |
| `C` | Change to EOL |
| `cw` | Change word |
| `yy` | Yank line |
| `p` | Paste after |
| `P` | Paste before |
| `>>` | Indent |
| `<<` | Dedent |
| `.` | Repeat last change |

### Text Objects

| Command | Scope |
|---------|-------|
| `iw` / `aw` | Inner/around word |
| `i"` / `a"` | Inner/around quotes |
| `i(` / `a(` | Inner/around parens |
| `i{` / `a{` | Inner/around braces |
| `i[` / `a[` | Inner/around brackets |

---

## Configuration Files

### Settings Location
- macOS: `~/.claude/`
- Linux: `~/.claude/`

### Key Files
```
~/.claude/settings.json     # User settings
~/.claude/CLAUDE.md         # Global memory/instructions
./CLAUDE.md                 # Project-specific instructions
```

### Settings Options
```json
{
  "model": "sonnet",
  "theme": "dark",
  "verbose": false,
  "permissions": {
    "allow": [],
    "deny": []
  }
}
```

---

## Workflow Patterns

### Exploration
```
What's the architecture of this codebase?
How does authentication work here?
```
Claude searches, reads, synthesizes.

### Implementation
```
Add rate limiting to the API endpoints
```
Claude plans, implements, shows diffs for approval.

### Debugging
```
The build fails with error X. Fix it.
```
Claude investigates, identifies cause, applies fix.

### Refactoring
```
Refactor @src/handlers.rs to use async/await
```
Claude reads, transforms, preserves behavior.

### Code Review
```
Review @src/payment.rs for security issues
```
Claude analyzes, reports findings.

---

## Piping & Scripting

### Non-interactive Use
```bash
# Single prompt, output to stdout
claude -p "Explain this error: $(cat error.log)"

# Pipe input
cat code.py | claude -p "Review this code"

# JSON output
claude -p "List files" --output-format json

# Streaming JSON
claude -p "Build feature X" --output-format stream-json
```

### Exit Codes
- `0` - Success
- Non-zero - Error (check stderr)

---

## Tips

1. **Use `@` liberally** - File references keep context precise
2. **`/compact` when slow** - Reduces context, speeds response
3. **`Ctrl+O` to debug** - See exactly what tools execute
4. **`/plan` for complex work** - Get alignment before execution
5. **Background long tasks** - `Ctrl+B` keeps you productive
6. **CLAUDE.md for patterns** - Teach project conventions once

---

## Quick Reference Card

```
Navigation          Session             Files
───────────         ───────             ─────
Ctrl+C  cancel      /clear   reset      @path  reference
Ctrl+D  exit        /resume  restore    !cmd   shell
Ctrl+L  clear       /export  save       /vim   editor
Ctrl+R  history     /compact shrink

Modes               Info                Config
─────               ────                ──────
/plan   planning    /context tokens     /config  settings
Ctrl+O  verbose     /cost    usage      /model   switch
Shift+Tab toggle    /tasks   bg jobs    /theme   colors
```
