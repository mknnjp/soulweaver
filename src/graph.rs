use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CommitNode {
    pub hash: String,
    pub parents: Vec<String>,
    pub subject: String,
    pub author: String,
    pub timestamp: i64,
    pub lane: usize,
    pub x: f32,
    pub y: f32,
}

pub struct GraphLayout {
    commits: Vec<CommitNode>,
    index_map: HashMap<String, usize>,
}

impl GraphLayout {
    pub fn new(commits: &[CommitNode]) -> Self {
        let index_map: HashMap<String, usize> = commits
            .iter()
            .enumerate()
            .map(|(i, c)| (c.hash.clone(), i))
            .collect();

        Self {
            commits: commits.to_vec(),
            index_map,
        }
    }

    pub fn assign_lanes(&mut self) {
        let mut active_lanes: Vec<Option<String>> = Vec::new();
        let row_height = 28.0;
        let lane_width = 24.0;
        let margin_x = 40.0;
        let margin_y = 20.0;

        for i in 0..self.commits.len() {
            let commit = self.commits[i].clone();

            let lane = if let Some(pos) = active_lanes
                .iter()
                .position(|l| l.as_deref() == Some(&commit.hash))
            {
                active_lanes[pos] = None;
                pos
            } else {
                if let Some(pos) = active_lanes.iter().position(|l| l.is_none()) {
                    pos
                } else {
                    active_lanes.push(None);
                    active_lanes.len() - 1
                }
            };

            for (pi, parent_hash) in commit.parents.iter().enumerate() {
                if pi == 0 {
                    active_lanes[lane] = Some(parent_hash.clone());
                } else {
                    if let Some(pos) = active_lanes.iter().position(|l| l.is_none()) {
                        active_lanes[pos] = Some(parent_hash.clone());
                    } else {
                        active_lanes.push(Some(parent_hash.clone()));
                    }
                }
            }

            let x = margin_x + (lane as f32) * lane_width;
            let y = margin_y + (i as f32) * row_height;

            self.commits[i].lane = lane;
            self.commits[i].x = x;
            self.commits[i].y = y;
        }
    }

    pub fn into_nodes(self) -> Vec<CommitNode> {
        self.commits
    }
}
