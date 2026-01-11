# FlowPrompt

[![Crates.io](https://img.shields.io/crates/v/flowprompt-cli.svg)](https://crates.io/crates/flowprompt-cli)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

```text
███████╗██╗      ██████╗ ██╗    ██╗██████╗ ██████╗  ██████╗ ███╗   ███╗██████╗ ████████╗
██╔════╝██║     ██╔═══██╗██║    ██║██╔══██╗██╔══██╗██╔═══██╗████╗ ████║██╔══██╗╚══██╔══╝
█████╗  ██║     ██║   ██║██║ █╗ ██║██████╔╝██████╔╝██║   ██║██╔████╔██║██████╔╝   ██║
██╔══╝  ██║     ██║   ██║██║███╗██║██╔═══╝ ██╔══██╗██║   ██║██║╚██╔╝██║██╔═══╝    ██║
██║     ███████╗╚██████╔╝╚███╔███╔╝██║     ██║  ██║╚██████╔╝██║ ╚═╝ ██║██║        ██║
╚═╝     ╚══════╝ ╚═════╝  ╚══╝╚══╝ ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝        ╚═╝
```

**FlowPrompt** is a powerful terminal-based tool designed to manage, categorize, and inject variables into Large Language Model (LLM) prompts without breaking your flow state.

Manage your prompts locally, pipe code directly into templates, and copy the result to your clipboard instantly—all from the comfort of your terminal.

---

## 🚀 Features

-   **Pipe-First Workflow**: `cat file.rs | flow use refactor` automatically fills `{{input}}`.
-   **Interactive TUI**: A "fun mode" dashboard (`flow ui`) to browse and select prompts with style.
-   **Smart Variables**: Automatically detects {{variables}} and prompts you for them interactively.
-   **Fuzzy Search**: Integrated `flow search` for when you forget prompt aliases.
-   **Local & Private**: 100% local JSON storage (`~/.config/flowprompt/prompts.json`).
-   **Clipboard Integration**: Output loops directly to your system clipboard (or stdout with `--print`).

## 📦 Installation

### From crates.io (Recommended)
You can install FlowPrompt directly via Cargo:

```bash
cargo install flowprompt-cli
```

### From Source

Ensure you have Rust installed (via [rustup](https://rustup.rs/)).

```bash
git clone https://github.com/pronzzz/flowprompt.git
cd flowprompt/flow
cargo install --path .
```

## 🛠 Usage

### 1. Create a Prompt (`flow add`)

Refining your prompts is key. Save them once, use them forever.

```bash
flow add
```

### 2. The Dashboard (`flow ui`)

Launch the interactive TUI to browse your collection.

```bash
flow ui
```

### 3. Quick Use (`flow use`)

**The Power Move**: Pipe content directly into a prompt.

```bash
cat src/main.rs | flow use refactor
```

### 4. Search (`flow search`)

Can't remember the alias? Fuzzy search it.

```bash
flow search
```

## 📂 Configuration

Prompts are stored in `~/.config/flowprompt/prompts.json`.

## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
