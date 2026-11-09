use crate::graph::{CommitNode, GitRef};

#[derive(Debug, Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Fetching,
    Connected { host: String, repo_path: String },
    Error(String),
}

#[derive(Default)]
#[allow(dead_code)]
pub struct AppState {
    pub host_input: String,
    pub port_input: String,
    pub user_input: String,
    pub key_path_input: String,
    pub repo_path_input: String,
    pub connection: ConnectionState,
    pub commits: Vec<CommitNode>,
    pub refs: Vec<GitRef>,
    pub scroll_offset: iced::widget::scrollable::RelativeOffset,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
}
