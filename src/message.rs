use crate::graph::{CommitNode, GitRef};

#[derive(Debug, Clone)]
pub enum Message {
    HostChanged(String),
    PortChanged(String),
    UserChanged(String),
    KeyPathChanged(String),
    RepoPathChanged(String),
    Connect,
    FetchStarted,
    FetchProgress(usize),
    FetchCompleted(Vec<CommitNode>, Vec<GitRef>),
    FetchFailed(String),
    CanvasEvent(iced::widget::canvas::Event),
    Scrolled(iced::widget::scrollable::RelativeOffset),
    ZoomIn,
    ZoomOut,
    ResetView,
    Close,
}
