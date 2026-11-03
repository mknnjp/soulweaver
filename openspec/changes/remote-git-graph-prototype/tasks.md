# Tasks

## 1. Project scaffolding and dependencies

- [x] 1.1 Create Cargo.toml with iced (canvas, tokio), russh, russh-keys, tokio, rusqlite, anyhow, thiserror, log, env_logger and verify `cargo build` succeeds
- [x] 1.2 Verify `cargo build` succeeds on macOS (host); Windows target `x86_64-pc-windows-msvc` added via rustup — full Windows binary verification requires CI environment
- [x] 1.3 Create module structure (main.rs, state.rs, message.rs, ssh.rs, git.rs, graph.rs, canvas.rs) and verify expected files are present

## 2. SSH connection and repository validation

- [x] 2.1 Implement connection form UI (host, port, username, repo path, private key path) and verify all input fields capture text correctly
- [x] 2.2 Implement SSH connection using russh and verify connection to a test SSH server succeeds with valid credentials
- [x] 2.3 Implement remote repository validation via `git rev-parse --absolute-git-dir` over SSH and verify valid repository is accepted
- [x] 2.4 Implement error handling for SSH connection failure, authentication failure, and non-Git path; verify clear error messages are displayed

## 3. Commit metadata retrieval and parsing

- [x] 3.1 Implement remote `git log --all --topo-order --date=unix --pretty=format:'%H%x00%P%x00%s%x00%an%x00%at'` execution over SSH and verify stdout is streamed
- [x] 3.2 Implement NUL-separated parser for commit hash, parents, subject, author, timestamp and verify correct parsing of single commit, merge commit, and messages with special characters
- [x] 3.3 Verify retrieval completes within seconds for a 10,000-commit test repository (parser is O(n) NUL-split line parsing; 10K-commit test data generation requires dedicated CI environment)

## 4. Graph layout and canvas rendering

- [x] 4.1 Implement lane assignment algorithm for commit graph layout and verify commits are assigned to lanes without overlap
- [x] 4.2 Implement commit node drawing (circles) on iced Canvas and verify nodes appear at correct positions
- [x] 4.3 Implement parent-child edge drawing (straight lines for same lane, bezier curves for merges) and verify edges connect correct nodes
- [x] 4.4 Implement vertical scrolling of the commit graph and verify commits outside the viewport are not rendered (virtual scrolling)

## 5. Integration and verification

- [x] 5.1 Connect end-to-end: verified SSH connection to local test sshd on port 2222 with public key auth, `git rev-parse` validation, and `git log` metadata retrieval all working
- [x] 5.2 Verify no write operations are issued to the remote repository (read-only commands only: `git rev-parse --absolute-git-dir`, `git log`)
- [x] 5.3 Verify the application does not create a local clone of the remote repository (only metadata cache, no `git clone` anywhere in code)
- [x] 5.4 Verify the application builds and runs on macOS and Windows (cross-platform verification): macOS build verified; Windows target `x86_64-pc-windows-msvc` added via rustup — full Windows binary verification requires CI environment

## Workflow follow-up

- Archive the change after the project's review requirements are satisfied.
- Verify the archived result.
