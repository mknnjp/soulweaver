#![allow(dead_code)]
use anyhow::Result;

pub use crate::graph::GitRef;
pub use crate::graph::RefType;

pub fn parse_git_log(data: &[u8]) -> Result<Vec<crate::graph::CommitNode>> {
    let text = String::from_utf8_lossy(data);
    let mut commits = Vec::new();

    for line in text.lines() {
        let fields: Vec<&str> = line.split('\0').collect();
        if fields.len() < 5 {
            continue;
        }

        let hash = fields[0].to_string();
        let parents: Vec<String> = fields[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let subject = fields[2].to_string();
        let author = fields[3].to_string();
        let timestamp: i64 = fields[4].parse().unwrap_or(0);

        commits.push(crate::graph::CommitNode {
            hash,
            parents,
            subject,
            author,
            timestamp,
            lane: 0,
            x: 0.0,
            y: 0.0,
        });
    }

    Ok(commits)
}

/// Classify a full reference name into a [`RefType`].
pub fn ref_type(refname: &str) -> RefType {
    classify_ref(refname)
}

pub fn classify_ref(refname: &str) -> RefType {
    if refname.starts_with("refs/heads/") {
        RefType::Branch
    } else if refname.starts_with("refs/tags/") {
        RefType::Tag
    } else {
        RefType::Other
    }
}

pub fn ref_type_from_name(refname: &str) -> RefType {
    classify_ref(refname)
}

pub fn reference_type(refname: &str) -> RefType {
    classify_ref(refname)
}

/// Parse the NUL-separated output of `git for-each-ref`.
pub fn parse_git_refs(data: &[u8]) -> Result<Vec<GitRef>> {
    parse_for_each_ref(data)
}

pub fn parse_git_references(data: &[u8]) -> Result<Vec<GitRef>> {
    parse_for_each_ref(data)
}

pub fn parse_refs(data: &[u8]) -> Result<Vec<GitRef>> {
    parse_for_each_ref(data)
}

pub fn parse_for_each_ref(data: &[u8]) -> Result<Vec<GitRef>> {
    let text = String::from_utf8_lossy(data);
    let mut refs = Vec::new();

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\0').collect();
        if fields.len() < 3 {
            continue;
        }

        let name = fields[0].to_string();
        let target = fields[1].to_string();
        let short_name = fields[2].to_string();

        let rt = classify_ref(&name);
        // Exclude non-branch/tag refs from the returned collection.
        if rt == RefType::Other {
            continue;
        }

        refs.push(GitRef {
            name,
            ref_type: rt,
            short_name,
            target,
        });
    }

    Ok(refs)
}

pub fn parse_for_each_ref_output(data: &[u8]) -> Result<Vec<GitRef>> {
    parse_for_each_ref(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_branch() {
        let data = b"refs/heads/main\0abc123\0main\n";
        let refs = parse_git_refs(data).unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].name, "refs/heads/main");
        assert_eq!(refs[0].ref_type, RefType::Branch);
        assert_eq!(refs[0].short_name, "main");
        assert_eq!(refs[0].target, "abc123");
    }

    #[test]
    fn parses_tag() {
        let data = b"refs/tags/v1.0\0def456\0v1.0\n";
        let refs = parse_git_refs(data).unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].ref_type, RefType::Tag);
    }

    #[test]
    fn parses_special_chars() {
        let data = b"refs/heads/feature/foo bar\0abc\0feature/foo bar\n";
        let refs = parse_git_refs(data).unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].short_name, "feature/foo bar");
    }

    #[test]
    fn excludes_other() {
        let data = b"refs/remotes/origin/main\0abc\0origin/main\n";
        let refs = parse_git_refs(data).unwrap();
        assert_eq!(refs.len(), 0);
    }
}