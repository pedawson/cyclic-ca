# Claude Code
## Mini Manual

---

## 1. Overview

Claude Code is a CLI-based AI assistant for software engineering tasks.

```
┌────────────┐    ┌────────────┐    ┌────────────┐    ┌────────────┐
│   INPUT    │ →  │  PROCESS   │ →  │   TOOLS    │ →  │   OUTPUT   │
│  (prompt)  │    │   (LLM)    │    │ (file/bash)│    │ (response) │
└────────────┘    └────────────┘    └────────────┘    └────────────┘
```

---

## 2. CLI Invocation

```
COMMAND                         DESCRIPTION
─────────────────────────────────────────────────────────────────────
claude                          Start interactive session
claude -c                       Continue most recent session
claude -r                       Resume picker (search past sessions)
claude -r <session-id>          Resume specific session
claude -p "prompt"              Non-interactive, print and exit
claude --model opus             Specify model (sonnet, opus, haiku)
claude --verbose                Show detailed tool output
claude --help                   Display all CLI options
```

---

## 3. CLI Flags Reference

```
FLAG                            DESCRIPTION
─────────────────────────────────────────────────────────────────────
-p, --print                     Non-interactive mode, output and exit
-c, --continue                  Continue most recent conversation
-r, --resume [id]               Resume session by ID or open picker
--model <model>                 Set model (sonnet, opus, haiku)
--system-prompt <text>          Override system prompt
--append-system-prompt <text>   Add to default system prompt
--allowed-tools <list>          Whitelist specific tools
--disallowed-tools <list>       Blacklist specific tools
--add-dir <path>                Grant access to additional directories
--verbose                       Show detailed execution
--debug                         Enable debug output
--mcp-config <file>             Load MCP server configuration
--permission-mode <mode>        Set permission handling mode
--output-format <format>        Output format (text, json, stream-json)
```

---

## 4. General Keyboard Controls

```
SHORTCUT                        ACTION
─────────────────────────────────────────────────────────────────────
Ctrl+C                          Cancel current operation
Ctrl+D                          Exit session (EOF)
Ctrl+L                          Clear screen (preserves history)
Ctrl+O                          Toggle verbose output
Ctrl+R                          Reverse search command history
Up/Down                         Navigate input history
Left/Right                      Cycle dialog tabs
Esc Esc                         Rewind conversation/code
Shift+Tab                       Cycle permission modes
Alt+M                           Cycle permission modes (alternate)
```

---

## 5. Text Editing Controls

```
SHORTCUT                        ACTION
─────────────────────────────────────────────────────────────────────
Ctrl+A                          Move to beginning of line
Ctrl+E                          Move to end of line
Ctrl+K                          Delete to end of line
Ctrl+U                          Delete entire line
Ctrl+Y                          Paste deleted text
Alt+Y                           Cycle paste history
Alt+B                           Move back one word
Alt+F                           Move forward one word
```

---

## 6. Session Management Controls

```
SHORTCUT                        ACTION
─────────────────────────────────────────────────────────────────────
Ctrl+G                          Open prompt in external editor
Ctrl+B                          Background current task
Ctrl+T                          Toggle task list view
Alt+P / Option+P                Switch model
Alt+T / Option+T                Toggle extended thinking
```

---

## 7. Multiline Input Methods

```
METHOD                          SHORTCUT
─────────────────────────────────────────────────────────────────────
Escape newline                  \ + Enter
macOS default                   Option+Enter
iTerm2/WezTerm/Kitty            Shift+Enter
Line feed character             Ctrl+J
```

---

## 8. Input Prefixes

```
PREFIX      FUNCTION                    EXAMPLE
─────────────────────────────────────────────────────────────────────
/           Slash command               /compact
!           Direct shell execution      !git status
@           File path autocomplete      @src/main.rs
```

---

## 9. Slash Commands - Session

```
COMMAND                         DESCRIPTION
─────────────────────────────────────────────────────────────────────
/clear                          Clear conversation history
/compact [focus]                Compress context, optional focus
/resume [session]               Resume previous session
/exit                           Exit Claude Code
/rename <name>                  Rename current session
/export [file]                  Export conversation (clipboard if no file)
```

---

## 10. Slash Commands - Information

```
COMMAND                         DESCRIPTION
─────────────────────────────────────────────────────────────────────
/help                           Show help
/context                        Visualize context window usage
/cost                           Show token usage and costs
/stats                          Daily usage statistics
/doctor                         Check installation health
/tasks                          List background tasks
/todos                          Show current todo items
/usage                          Show plan limits (subscription)
```

---

## 11. Slash Commands - Configuration

```
COMMAND                         DESCRIPTION
─────────────────────────────────────────────────────────────────────
/config                         Open settings interface
/model                          Change model
/permissions                    View/edit permissions
/theme                          Change color theme
/statusline                     Configure status line
/memory                         Edit CLAUDE.md files
/mcp                            Manage MCP servers
/vim                            Enable vim editing mode
/plan                           Enter plan mode
/init                           Create CLAUDE.md project guide
```

---

## 12. Operating Modes

```
MODE                ACTIVATION              BEHAVIOR
─────────────────────────────────────────────────────────────────────
Normal              Default                 Asks permission for edits/commands
Plan                /plan                   Read-only, produces plan for approval
Auto-Accept         Shift+Tab to cycle      Bypasses permission prompts
Verbose             Ctrl+O                  Shows full tool execution details
```

---

## 13. Context Management

```
ACTION                          METHOD
─────────────────────────────────────────────────────────────────────
Reference file                  @path/to/file
Reference multiple files        @file1.rs @file2.rs
Reference line range            @file.rs:50-100
View token usage                /context
Compress context                /compact
Set compression focus           /compact focus on auth logic
Edit project memory             /memory
```

---

## 14. Background Operations

```
ACTION                          METHOD
─────────────────────────────────────────────────────────────────────
Background current task         Ctrl+B
List background tasks           /tasks
View task output                Select from /tasks menu
```

---

## 15. MCP Server Management

```
COMMAND                         DESCRIPTION
─────────────────────────────────────────────────────────────────────
claude mcp list                 Show configured servers
claude mcp add <name> <cmd>     Add server
claude mcp remove <name>        Remove server
/mcp                            In-session management
```

---

## 16. Configuration Files

```
FILE                            PURPOSE
─────────────────────────────────────────────────────────────────────
~/.claude/settings.json         User settings
~/.claude/CLAUDE.md             Global memory/instructions
./CLAUDE.md                     Project-specific instructions
```

---

## 17. VS Code Integration

```
SHORTCUT                        ACTION
─────────────────────────────────────────────────────────────────────
Cmd+Esc / Ctrl+Esc              Toggle focus to Claude panel
Cmd+Shift+Esc / Ctrl+Shift+Esc  Open in new tab
Cmd+N / Ctrl+N                  New conversation
Option+K / Alt+K                Insert file reference with line numbers
```

---

## 18. JetBrains Integration

```
SHORTCUT                        ACTION
─────────────────────────────────────────────────────────────────────
Cmd+Esc / Ctrl+Esc              Open Claude panel
Cmd+Option+K / Alt+Ctrl+K       Insert file reference
```

---

## 19. Vim Mode - Activation

```
COMMAND                         ACTION
─────────────────────────────────────────────────────────────────────
/vim                            Enable vim mode
Esc                             Enter NORMAL mode (from INSERT)
i                               Insert before cursor
I                               Insert at line start
a                               Insert after cursor
A                               Insert at line end
o                               Open line below
O                               Open line above
```

---

## 20. Vim Mode - Navigation

```
COMMAND                         MOTION
─────────────────────────────────────────────────────────────────────
h / j / k / l                   Left / down / up / right
w                               Next word
b                               Previous word
e                               End of word
0                               Line start
$                               Line end
^                               First non-blank character
gg                              Document start
G                               Document end
f{char}                         Jump to character
F{char}                         Jump back to character
```

---

## 21. Vim Mode - Editing

```
COMMAND                         ACTION
─────────────────────────────────────────────────────────────────────
x                               Delete character
dd                              Delete line
D                               Delete to end of line
dw                              Delete word
cc                              Change line
C                               Change to end of line
cw                              Change word
yy / Y                          Yank (copy) line
yw                              Yank word
p                               Paste after cursor
P                               Paste before cursor
>>                              Indent line
<<                              Dedent line
.                               Repeat last change
```

---

## 22. Vim Mode - Text Objects

```
COMMAND                         SCOPE
─────────────────────────────────────────────────────────────────────
iw / aw                         Inner / around word
i" / a"                         Inner / around double quotes
i' / a'                         Inner / around single quotes
i( / a(                         Inner / around parentheses
i{ / a{                         Inner / around braces
i[ / a[                         Inner / around brackets
```

---

## 23. Piping & Scripting

```
COMMAND                              DESCRIPTION
─────────────────────────────────────────────────────────────────────
claude -p "prompt"                   Single prompt, output to stdout
cat file | claude -p "review"        Pipe input to Claude
claude -p "list" --output-format json    JSON output
claude -p "build" --output-format stream-json   Streaming JSON
```

---

## 24. Quick Reference Card

```
─────────────────────────────────────────────────────────────────────
NAVIGATION                      SESSION
  Ctrl+C    cancel                /clear     reset history
  Ctrl+D    exit                  /resume    restore session
  Ctrl+L    clear screen          /export    save conversation
  Ctrl+R    search history        /compact   compress context

TEXT EDITING                    MODES
  Ctrl+K    delete to EOL         /plan      planning mode
  Ctrl+U    delete line           Ctrl+O     verbose mode
  Ctrl+Y    paste                 Shift+Tab  cycle permissions
  Alt+B/F   word movement         /vim       vim editing

FILES                           CONFIG
  @path     reference file        /config    settings
  !cmd      shell command         /model     switch model
  /         slash command         /theme     change theme

IDE SHORTCUTS
  Cmd+Esc       toggle Claude panel
  Option+K      insert file reference
─────────────────────────────────────────────────────────────────────
```
