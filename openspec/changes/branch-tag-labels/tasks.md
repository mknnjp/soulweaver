# Tasks

## 1. Reference data model and parser

- [ ] 1.1 Define a `GitRef` struct (reference name, ref type branch/tag, short display name, target commit hash) in `src/graph.rs` and verify `cargo build` succeeds
- [ ] 1.2 Implement a NUL-separated parser for `git for-each-ref` output in `src/git.rs` (accepting `%(refname)%x00%(objectname)%x00%(refname:short)` output) and verify with unit tests that a branch, a tag, and a ref with special characters each parse into the correct name, type, short name, and target hash
- [ ] 1.3 Verify ref type derivation handles `refs/heads/*` as branch, `refs/tags/*` as tag, and other namespaces (`refs/remotes/*`) by unit test, deciding explicitly how non-branch/tag refs are classified or excluded

## 2. Reference retrieval over SSH

- [ ] 2.1 Extend the SSH flow in `src/ssh.rs` to run `git for-each-ref --format='%(refname)%x00%(objectname)%x00%(refname:short)'` after repository validation and verify execution against a local test sshd returns output that the new parser accepts
- [ ] 2.2 Handle a non-zero exit status from `git for-each-ref` with a clear error (surfaced to the user) and verify by running the command against a path that is a valid repository, checking the error path is exercised rather than panicking
- [ ] 2.3 Return parsed references alongside commits from `fetch_git_graph` (or an equivalent entry point) and verify the shape of the returned data compiles and carries both commits and refs

## 3. Mapping references to commits

- [ ] 3.1 Map each parsed reference to the commit node with the matching hash while holding commits in application state, and verify with a unit test that references bind to the correct nodes and unmapped refs are dropped without error
- [ ] 3.2 Store the references in `AppState` (`src/state.rs`) and add any `Message` variants in `src/message.rs` needed to deliver fetched references to the update function, verifying `cargo build` succeeds and the fields are populated in `update`

## 4. Canvas label rendering

- [ ] 4.1 Draw a branch/tag label (rounded "pill" with text) to the right of each commit node that has references in `src/canvas.rs`, and verify visually against a test repository that labels appear next to the correct nodes
- [ ] 4.2 Apply distinct visual styling for branch labels versus tag labels (different colors or badge styles) and verify both appear distinguishable in the rendered canvas
- [ ] 4.3 Handle label placement inside the visible viewport only (labels for nodes outside the view are not drawn) and verify scrolling behavior keeps labels aligned with their nodes without affecting scroll performance
- [ ] 4.4 Guard against a commit with many references overflowing the canvas (clamp or truncate label width) and verify with a test repository containing many refs on one commit that the graph remains usable

## 5. Integration and verification

- [ ] 5.1 Connect end-to-end against a local test sshd serving a repository with branches and tags, and verify references display as labels on the correct commits in the running app
- [ ] 5.2 Verify the read-only contract still holds: no write commands are issued, and confirm the added command is `git for-each-ref` only (code review and command audit)
- [ ] 5.3 Verify `cargo build` succeeds with no warnings introduced by the reference feature, and run any new unit tests to confirm they pass

## Workflow follow-up

- Archive the change after the project's review requirements are satisfied.
- Verify the archived result.
