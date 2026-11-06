mod canvas;
mod git;
mod graph;
mod message;
mod ssh;
mod state;

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length, Task, Theme};
use message::Message;
use state::AppState;

fn main() -> iced::Result {
    iced::application("Remote Git Graph Viewer", update, view)
        .theme(|_| Theme::Dark)
        .window_size((1280.0, 800.0))
        .run()
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::HostChanged(v) => {
            state.host_input = v;
            Task::none()
        }
        Message::PortChanged(v) => {
            state.port_input = v;
            Task::none()
        }
        Message::UserChanged(v) => {
            state.user_input = v;
            Task::none()
        }
        Message::KeyPathChanged(v) => {
            state.key_path_input = v;
            Task::none()
        }
        Message::RepoPathChanged(v) => {
            state.repo_path_input = v;
            Task::none()
        }
        Message::Connect => {
            state.connection = state::ConnectionState::Connecting;

            let host = state.host_input.clone();
            let port = state.port_input.clone();
            let user = state.user_input.clone();
            let key_path = state.key_path_input.clone();
            let repo_path = state.repo_path_input.clone();

            Task::perform(
                async move {
                    ssh::fetch_git_graph(&host, &port, &user, &key_path, &repo_path).await
                },
                |result| match result {
                    Ok(commits) => Message::FetchCompleted(commits),
                    Err(e) => Message::FetchFailed(e.to_string()),
                },
            )
        }
        Message::FetchCompleted(commits) => {
            let mut layout = graph::GraphLayout::new(&commits);
            layout.assign_lanes();
            state.commits = layout.into_nodes();
            state.connection = state::ConnectionState::Connected {
                host: state.host_input.clone(),
                repo_path: state.repo_path_input.clone(),
            };
            Task::none()
        }
        Message::FetchFailed(e) => {
            state.connection = state::ConnectionState::Error(e);
            Task::none()
        }
        Message::CanvasEvent(_) => Task::none(),
        Message::Scrolled(_) => Task::none(),
        Message::ZoomIn => {
            state.zoom = (state.zoom + 0.2).clamp(0.1, 5.0);
            Task::none()
        }
        Message::ZoomOut => {
            state.zoom = (state.zoom - 0.2).clamp(0.1, 5.0);
            Task::none()
        }
        Message::ResetView => {
            state.zoom = 1.0;
            state.pan_x = 0.0;
            state.pan_y = 0.0;
            Task::none()
        }
        Message::Close => iced::exit(),
        Message::FetchStarted | Message::FetchProgress(_) => Task::none(),
    }
}

fn view(state: &AppState) -> Element<Message> {
    let connection_form = column![
        text("Remote Git Graph Viewer").size(20),
        row![
            text("User:"),
            text_input("user", &state.user_input)
                .on_input(Message::UserChanged)
                .width(100),
            text("@"),
            text_input("host", &state.host_input)
                .on_input(Message::HostChanged)
                .width(150),
            text(":"),
            text_input("22", &state.port_input)
                .on_input(Message::PortChanged)
                .width(40),
            text("Key:"),
            text_input("~/.ssh/id_ed25519", &state.key_path_input)
                .on_input(Message::KeyPathChanged)
                .width(150),
            text("Repo:"),
            text_input("/path/to/repo", &state.repo_path_input)
                .on_input(Message::RepoPathChanged)
                .width(Length::Fill),
            button("Connect")
                .on_press(Message::Connect)
                .padding([6, 16]),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(12);

    let status = match &state.connection {
        state::ConnectionState::Disconnected => "Disconnected".to_string(),
        state::ConnectionState::Connecting => "Connecting...".to_string(),
        state::ConnectionState::Fetching => "Fetching git log...".to_string(),
        state::ConnectionState::Connected { host, repo_path } => {
            format!(
                "Connected: {}:{} ({} commits)",
                host,
                repo_path,
                state.commits.len()
            )
        }
        state::ConnectionState::Error(e) => format!("Error: {}", e),
    };

    let graph_canvas = iced::widget::canvas(canvas::GitGraphCanvas::new(
        &state.commits,
    ))
    .width(Length::Fill)
    .height(Length::Fill);

    let canvas_with_scroll = scrollable(graph_canvas)
        .on_scroll(|viewport| Message::Scrolled(viewport.relative_offset()))
        .width(Length::Fill)
        .height(Length::Fill);

    let toolbar = row![
        button("+").on_press(Message::ZoomIn),
        button("-").on_press(Message::ZoomOut),
        button("Reset").on_press(Message::ResetView),
        text(format!("Zoom: {:.1}x", state.zoom)).size(14),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    column![
        container(connection_form).padding(12),
        container(toolbar).padding([0, 12]),
        container(canvas_with_scroll).padding(12),
        container(text(status).size(13)).padding([4, 12]),
    ]
    .spacing(8)
    .into()
}
