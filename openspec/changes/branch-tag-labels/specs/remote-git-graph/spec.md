# Spec Delta

## ADDED Requirements

### Requirement: Reference list retrieval

The system SHALL retrieve branch and tag references from the remote repository using `git for-each-ref` over SSH, without downloading file contents or performing any write operation on the remote repository.

#### Scenario: Successful reference retrieval

- **GIVEN** an SSH connection has been established and the remote repository is valid
- **WHEN** reference retrieval is triggered
- **THEN** the system retrieves branch and tag references, each with its reference name and target commit hash, via `git for-each-ref`

#### Scenario: Distinction of reference types and targets

- **GIVEN** reference retrieval succeeds
- **WHEN** the system parses the `git for-each-ref` output
- **THEN** each reference is identified as either a branch or a tag, and its short display name is derived from its reference name

#### Scenario: Reference retrieval failure

- **GIVEN** an SSH connection has been established and the remote repository is valid
- **WHEN** `git for-each-ref` exits with an error status
- **THEN** the system surfaces a clear error and does not render the completed graph as if references were successfully loaded

### Requirement: Mapping references to commit hashes

The system SHALL map each retrieved reference to the commit node whose hash matches the reference target, so that a reference points to exactly the commit it targets.

#### Scenario: Associating references with commits

- **GIVEN** commits are retrieved and references are retrieved
- **WHEN** the system maps references to commits by target commit hash
- **THEN** each reference that targets a retrieved commit is associated with that commit node

#### Scenario: References pointing outside the fetched range

- **GIVEN** a retrieved reference targets a commit hash that is not in the retrieved commit list
- **WHEN** the system maps references to commits by target commit hash
- **THEN** that reference is not attached to any commit node and does not cause an error

#### Scenario: Multiple references pointing to the same commit

- **GIVEN** multiple references target the same commit hash
- **WHEN** the system maps references to commits by target commit hash
- **THEN** all of those references are associated with that single commit node

### Requirement: Branch and tag label rendering

The system SHALL draw each branch and tag label next to its associated commit node on the graph canvas, distinguishing branches from tags visually so the user can identify which kind of reference each label is.

#### Scenario: Label rendering

- **GIVEN** commit metadata and references are retrieved and mapped
- **WHEN** graph rendering is triggered
- **THEN** the system draws each branch and tag label next to the commit node it is associated with

#### Scenario: Distinguishing branches and tags

- **GIVEN** a commit has both a branch reference and a tag reference associated with it
- **WHEN** the system draws the labels
- **THEN** the branch label and the tag label are visually distinguishable from each other

#### Scenario: Scrolling graph with labels

- **GIVEN** the graph with labels has been rendered and exceeds the visible area
- **WHEN** the user scrolls vertically
- **THEN** labels move with their commit nodes, and labels outside the viewport are not rendered
