use crate::graph::CommitNode;
use anyhow::Result;

pub fn parse_git_log(data: &[u8]) -> Result<Vec<CommitNode>> {
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

        commits.push(CommitNode {
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
