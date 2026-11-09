# remote-git-graph Specification

## Purpose

Lets users view the commit graph of a Git repository on a remote SSH server without cloning it locally, using a lightweight cross-platform desktop application.

## Requirements

### Requirement: SSH connection to remote repository

The system SHALL allow a user to connect to a remote Git repository over SSH by providing host, port, username, repository path, and private key path, without requiring any agent or daemon to be installed on the remote server.

#### Scenario: Successful SSH connection
- **GIVEN** the user has entered host, port, username, repository path, and private key path, and the remote host is reachable
- **WHEN** the user clicks Connect
- **THEN** the system establishes an SSH connection to the remote host and authenticates with the provided private key

#### Scenario: Authentication failure
- **GIVEN** the user has entered invalid SSH credentials and the remote host is reachable
- **WHEN** the user clicks Connect
- **THEN** the system displays a clear error message indicating authentication failure

### Requirement: Remote Git repository validation

The system SHALL verify that the specified path on the remote server is a valid Git repository before attempting to read its history.

#### Scenario: Valid Git repository
- **GIVEN** an SSH connection to the remote server has been established
- **WHEN** the remote path contains a valid `.git` directory or is a bare repository
- **THEN** the system proceeds to retrieve commit data

#### Scenario: Invalid path
- **GIVEN** an SSH connection to the remote server has been established
- **WHEN** the remote path is not a Git repository
- **THEN** the system displays a clear error message stating that the path is not a Git repository

### Requirement: Commit metadata retrieval

The system SHALL retrieve commit hashes, parent hashes, subjects, authors, and timestamps from the remote repository using `git log` over SSH, without downloading file contents or the full repository.

#### Scenario: Metadata retrieval
- **GIVEN** an SSH connection has been established and the remote repository is valid
- **WHEN** commit metadata retrieval is triggered
- **THEN** the system retrieves commit metadata via `git log --all --topo-order` and parses NUL-separated output

#### Scenario: Large repository
- **GIVEN** an SSH connection has been established and the remote repository is valid
- **WHEN** the remote repository contains tens of thousands of commits and metadata retrieval is triggered
- **THEN** the system retrieves and parses the metadata without blocking the UI indefinitely

### Requirement: Commit graph visualization

The system SHALL render the retrieved commit metadata as a visual graph on a canvas, showing commit nodes, parent-child edges, branches, and merges.

#### Scenario: Graph rendering
- **GIVEN** commit metadata has been retrieved
- **WHEN** graph rendering is triggered
- **THEN** the system draws commit nodes and edges connecting parents to children on the canvas

#### Scenario: Scrollable graph
- **GIVEN** the commit graph has been rendered and the number of commits exceeds the visible area
- **WHEN** the user scrolls vertically
- **THEN** additional commits become visible and commits outside the viewport are not rendered

### Requirement: Read-only remote access

The system SHALL not perform any write operations on the remote repository; it only reads commit metadata and references.

#### Scenario: Read-only enforcement
- **GIVEN** an SSH connection has been established
- **WHEN** Git commands are issued to the remote server
- **THEN** only read-only commands (e.g., `git log`, `git for-each-ref`, `git rev-parse`) are issued and no write commands are executed
