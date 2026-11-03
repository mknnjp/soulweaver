# Design

## Context

This is Phase 1 of the Remote Git Graph Viewer project. No existing specs cover this capability. The application is a cross-platform desktop app built with Rust and iced (macOS, Windows, Linux). It connects to a remote host over SSH, executes read-only Git commands, parses the output, and renders a commit graph on a canvas. See proposal.md for motivation.

## Goals / Non-Goals

**Goals:**
- Connect to a remote Git repository over SSH using host, port, username, repository path, and private key
- Validate that the remote path is a Git repository
- Retrieve commit metadata (hash, parents, subject, author, timestamp) via `git log` over SSH
- Render commit nodes and parent-child edges on an iced Canvas
- Scroll through the commit graph vertically
- Keep all remote operations read-only

**Non-Goals (Phase 2+):**
- Local SQLite caching and incremental/delta updates
- Zoom and pan operations
- Branch and tag label display on the graph
- Commit detail panel
- `~/.ssh/config` parsing and SSH agent integration
- Password authentication
- Source code diff or file content viewing

## Decisions

### iced (Rust) over Tauri / Electron

**Decision:** Use iced as the UI framework.

**Rationale:** The user explicitly requested native UI without WebView/Electron/Tauri. iced provides GPU-accelerated canvas rendering via wgpu, produces a single lightweight binary, and suits the "single-purpose lightweight tool" concept.

**Alternatives considered:**
- Tauri (WebView): Rejected - user wants native UI, no WebView
- GTK4 (gtk-rs): Rejected for Phase 1 - iced has simpler canvas API and faster iteration for prototyping
- Qt6 (C++): Rejected - higher development cost and larger binary

### Agentless SSH with russh

**Decision:** Use russh (Pure Rust async SSH) to connect and execute commands. No remote agent/daemon is installed.

**Rationale:** The core requirement is to avoid installing anything on the remote server. russh integrates with tokio for non-blocking streaming of `git log` output.

**Alternatives considered:**
- System `ssh` binary via `std::process::Command`: Simpler but harder to stream stdout asynchronously and parse incrementally
- libssh2 bindings: More C interop complexity; russh is Pure Rust

### NUL-separated `git log` output

**Decision:** Retrieve commit data using `git log --all --topo-order --date=unix --pretty=format:'%H%x00%P%x00%s%x00%an%x00%at'`.

**Rationale:** NUL separators safely handle commit messages and author names containing newlines or special characters. `--topo-order` guarantees children appear before parents, which is needed for graph layout.

**Alternatives considered:**
- JSON output from `git log`: Git does not natively output JSON for log
- Default pretty format with whitespace splitting: Unsafe for messages with spaces/newlines

### Canvas-based graph rendering

**Decision:** Use iced's `widget::canvas` to draw commit nodes and edges.

**Rationale:** Canvas provides direct 2D drawing control, suitable for rendering thousands of nodes with virtualization. Custom bezier curves can be drawn for merge edges.

**Alternatives considered:**
- SVG overlay: More complex to manage with scrolling and zooming
- HTML/CSS-based graph: Not applicable (no WebView)

### Virtual scrolling (visible-range-only rendering)

**Decision:** Only render commits within the visible viewport range, not the full commit list.

**Rationale:** Repositories with 10,000+ commits cannot be rendered all at once without freezing the UI.

## Risks / Trade-offs

[Risk] Large repositories (100,000+ commits) may still cause memory or parsing delays → Mitigation: Stream parsing, visible-range rendering, progress indicator

[Risk] iced Canvas IME (Japanese input) support may be incomplete → Mitigation: Connection form uses iced text_input (not Canvas); defer Canvas text input to Phase 2+

[Risk] russh host key verification (known_hosts) not implemented in Phase 1 → Mitigation: Accept host key on first connection with warning; add known_hosts checking in Phase 2+

[Risk] Merge edges with many parents (octopus merges) may look cluttered → Mitigation: Simple bezier curves; improve lane algorithm in Phase 2+

## Open Questions

- Should the private key passphrase be supported in Phase 1, or deferred?
- Should `.ssh/config` parsing be added before or after MVP?
- What is the maximum commit count target for Phase 1 performance (10k? 50k? 100k?)
