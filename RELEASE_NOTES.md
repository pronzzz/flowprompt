# FlowPrompt v1.0.0 Release Notes

We are thrilled to announce the first major release of **FlowPrompt**! 🚀

FlowPrompt is a terminal-centric tool for developers who want to manage their LLM prompts without leaving their CLI.

## 🌟 Key Features

*   **Pipeline Architecture**: Designed for Unix pipes. `cat code.rs | flow use refactor` just works.
*   **Interactive TUI**: A beautiful terminal dashboard (`flow ui`) using Ratatui for browsing prompts.
*   **Contextual Variables**: Automatically detects variables in templates (e.g., `{{language}}`) and prompts you for them.
*   **Fuzzy Search**: integrated `skim` support via `flow search`.
*   **Clipboard Integration**: Instantly places the formatted prompt into your system clipboard, ready to paste into ChatGPT/Claude.

## 📦 Changes in this Release

*   Initial release of the `flow` binary.
*   Implemented JSON storage backend.
*   Added `add`, `list`, `use`, `search`, and `ui` commands.
*   Full cross-platform clipboard support.

## 🚀 Getting Started

```bash
git clone https://github.com/pronzzz/flowprompt.git
cd flowprompt/flow
cargo install --path .
```
