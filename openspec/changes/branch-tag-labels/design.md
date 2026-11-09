# Design

## Context

See proposal.md - Why. The current Remote Git Graph Viewer retrieves commit logs via `git log` over SSH and renders nodes and edges on an iced Canvas. It lacks any display of branch and tag references pointing to commits. This design outlines how to fetch remote references using `git for-each-ref`, map them to commit hashes, and render them as labeled pills next to commit nodes on the canvas.

## Goals / Non-Goals

**Goals:**
- Fetch local/remote branch and tag references safely via `git for-each-ref` over SSH (read-only)
- Parse reference name, type (branch/tag), and target commit hash
- Associate references with their target commit nodes in the application state and graph layout
- Render branch and tag labels (pills) next to commit nodes on the iced Canvas with distinct styling
- Maintain smooth vertical scrolling and performance

**Non-Goals:**
- Creating, switching, deleting, or pushing branches/tags (strictly read-only)
- Interactive branch/tag actions (checkout, merge, diff from UI) in this phase
- Complex ref filtering UI (all refs returned by `git for-each-ref` are displayed)

## Decisions

### 1. Fetching references via `git for-each-ref`

**Decision:** Execute `git for-each-ref --format='%(refname)%x00%(objectname)%x00%(refname:short)'` over SSH in parallel with or sequentially following `git log`.

**Rationale:** 
- `git for-each-ref` provides a clean, scriptable way to list all refs (heads, tags, remotes) with their target object hashes.
- NUL-separated formatting (`%x00`) avoids any quoting or whitespace issues with ref names.
- It is a standard, lightweight read-only git command that requires no server-side agent installation.

**Alternatives considered:**
- Parsing `git show-ref` or `git branch -a` / `git tag`: Less uniform format across branches and tags compared to `git for-each-ref`.

### 2. Reference mapping in graph structure

**Decision:** Store a `Vec<GitRef>` in `AppState` or attach references to `CommitNode` during the layout/mapping phase.

**Rationale:**
- Each commit node can look up its associated references quickly by matching its commit hash against reference target hashes.
- Storing references alongside commits enables clean separation of concerns in `graph.rs`.

**Alternatives considered:**
- Storing a separate `HashMap<String, Vec<GitRef>>` mapped by commit hash in canvas state: Also viable, but storing references directly in or associated with `CommitNode` simplifies rendering.

### 3. Canvas label rendering & styling

**Decision:** Render labels as rounded rectangles (pills) with text inside, positioned to the right of each commit node circle in `canvas.rs`. Use different colors or badge styles for branches vs. tags (e.g., blue/cyan for branches, orange/yellow for tags).

**Rationale:**
- iced Canvas supports drawing filled paths, rounded rectangles, and text.
- Visual distinction between branches and tags helps users immediately identify ref types.
- Placing labels to the right of commit nodes prevents overlap with parent-child lines (lanes).

**Alternatives considered:**
- Overlaying iced native widgets (like `text` or `badge`) on top of the canvas: Difficult to synchronize precisely with virtual scrolling and canvas panning/zooming. Canvas-native drawing ensures rigid alignment.

## Risks / Trade-offs

- [Risk] **Label crowding**: If a commit has many branches and tags pointing to it, labels may overflow horizontally → *Mitigation*: Limit label width/truncation, wrap or stack labels if necessary in future iterations, or prioritize branches over tags when space is tight.
- [Risk] **Detached / unreachable refs**: Refs pointing to commits outside the fetched `git log --all` range → *Mitigation*: Unmapped refs are safely ignored without crashing the application.
