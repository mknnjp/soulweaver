use crate::graph::CommitNode;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Fetching,
    Connected { host: String, repo_path: String },
    Error(String),
}

#[derive(Default)]
pub struct AppState {
    pub host_input: String,
    pub port_input: String,
    pub user_input: String,
    pub key_path_input: String,
    pub repo_path_input: String,
    pub connection: ConnectionState,
    pub commits: Vec<CommitNode>,
    pub scroll_offset: iced::widget::scrollable::RelativeOffset,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
}
