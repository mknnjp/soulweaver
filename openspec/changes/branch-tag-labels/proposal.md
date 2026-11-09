# Proposal

## Why

The current Remote Git Graph Viewer (prototype) only displays commit graph nodes and edges without showing which commits are pointed to by branches and tags. Consequently, users cannot identify current branch positions or release tags from the graph alone, making it difficult to understand the context of commit history. Displaying branch and tag labels is necessary to provide an at-a-glance overview of the remote repository state.

## What Changes

- Add functionality to retrieve remote reference lists (branches and tags) using `git for-each-ref` over SSH
- Map retrieved references to commit hashes and associate them with their corresponding commit nodes
- Render branch and tag labels next to commit nodes on the iced Canvas (including color-coding and distinction)
- Ensure label rendering does not interfere with existing graph drawing and scrolling behavior

## Capabilities

### New Capabilities

None — no new standalone capabilities are introduced.

### Modified Capabilities

- `remote-git-graph`: Add requirements for retrieving references (`git for-each-ref`) and rendering branch and tag labels to the existing "Commit graph visualization" capability. "Read-only remote access" remains strictly enforced.

## Impact

- **src/git.rs**: Add a parser for `for-each-ref` output (extracting ref type, short name, and target hash)
- **src/ssh.rs**: Integrate reference retrieval (`git for-each-ref`) into the `fetch_git_graph` flow
- **src/graph.rs**: Add reference-to-commit hash mapping structures and label storage
- **src/canvas.rs**: Render labels next to nodes (iced Canvas text rendering, color coding, layout)
- **src/state.rs / src/message.rs / src/main.rs**: Hold, pass, and update reference data states
- Note: In the current checkout, `openspec/specs/` is empty because the prototype archive is pending in the main checkout. This change builds upon that prototype delta (`remote-git-graph`).
