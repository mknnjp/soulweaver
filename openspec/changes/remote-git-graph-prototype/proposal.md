# Proposal

## Why

There is no lightweight way to visually browse the commit history of a Git repository on a remote SSH server without cloning it locally or launching a heavy IDE such as VS Code Remote-SSH. Existing Git GUI clients assume a local clone, and the terminal `git log --graph` has limitations in visibility and operability. Infrastructure engineers, SREs, and developers working with large repositories need a standalone cross-platform desktop app to quickly inspect deployed repositories and bare repositories on servers.

## What Changes

- Create a new cross-platform desktop application using Rust and iced (macOS, Windows, Linux)
- Execute `git log` against a remote repository over SSH and retrieve commit metadata (agentless)
- Render the retrieved commit graph on a Canvas, scrollable/zoomable/pannable for browsing
- Speed up subsequent views using a local cache (SQLite)
- Do not create a full local clone of the Git repository (metadata cache only)

## Capabilities

### New Capabilities

- `remote-git-graph`: Ability to visualize and browse commit graphs of remote Git repositories over SSH. A read-only history viewer that does not require a full clone.

### Modified Capabilities

- None

## Impact

- New Rust desktop application (binary distribution)
- Dependency libraries: iced, russh, rusqlite, anyhow, thiserror, tokio
- Configuration files: `~/.ssh/config` integration (Phase 2+)
- No installation required on the remote server side (SSH + git commands only)
