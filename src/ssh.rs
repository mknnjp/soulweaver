use crate::git::{parse_git_log, parse_git_refs};
use crate::graph::{CommitNode, GitRef};
use anyhow::{Context, Result};
use russh::client::{self, Handler};
use russh::ChannelMsg;
use std::sync::Arc;

pub async fn fetch_git_graph(
    host: &str,
    port: &str,
    user: &str,
    key_path: &str,
    repo_path: &str,
) -> Result<(Vec<CommitNode>, Vec<GitRef>)> {
    let addr = if port.is_empty() {
        host.to_string()
    } else {
        format!("{}:{}", host, port)
    };

    let config = Arc::new(client::Config::default());
    let handler = ClientHandler;

    let mut session = client::connect(config, &addr, handler)
        .await
        .context("SSH connection failed")?;

    let key = russh_keys::load_secret_key(key_path, None)
        .context("Failed to load secret key")?;

    session
        .authenticate_publickey(user, key.into())
        .await
        .context("SSH authentication failed")?;

    // Validate that the remote path is a Git repository
    let mut channel = session.channel_open_session().await?;
    let validate_cmd = format!("git -C {} rev-parse --absolute-git-dir", repo_path);
    channel.exec(true, validate_cmd.as_str()).await?;

    let mut validate_output = Vec::new();
    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { data } => {
                validate_output.extend_from_slice(&data);
            }
            ChannelMsg::ExitStatus { exit_status } => {
                if exit_status != 0 {
                    anyhow::bail!(
                        "Path '{}' is not a valid Git repository",
                        repo_path
                    );
                }
                break;
            }
            ChannelMsg::Eof => break,
            _ => {}
        }
    }

    // Retrieve commit metadata
    let mut channel = session.channel_open_session().await?;
    let cmd = format!(
        "cd {} && git log --all --topo-order --date=unix \
         --pretty=format:'%H%x00%P%x00%s%x00%an%x00%at'",
        repo_path
    );

    channel.exec(true, cmd.as_str()).await?;

    let mut raw_output = Vec::new();

    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { data } => {
                raw_output.extend_from_slice(&data);
            }
            ChannelMsg::ExitStatus { exit_status } => {
                if exit_status != 0 {
                    anyhow::bail!("git log exited with code {}", exit_status);
                }
                break;
            }
            ChannelMsg::Eof => break,
            _ => {}
        }
    }

    let commits = parse_git_log(&raw_output)?;

    // Retrieve references
    let mut channel = session.channel_open_session().await?;
    let ref_cmd = format!(
        "git -C {} for-each-ref --format='%(refname)%x00%(objectname)%x00%(refname:short)'",
        repo_path
    );
    channel.exec(true, ref_cmd.as_str()).await?;

    let mut ref_output = Vec::new();
    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { data } => {
                ref_output.extend_from_slice(&data);
            }
            ChannelMsg::ExitStatus { exit_status } => {
                if exit_status != 0 {
                    anyhow::bail!("git for-each-ref exited with code {}", exit_status);
                }
                break;
            }
            ChannelMsg::Eof => break,
            _ => {}
        }
    }

    let refs = parse_git_refs(&ref_output)?;
    Ok((commits, refs))
}

struct ClientHandler;

#[async_trait::async_trait]
impl Handler for ClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh_keys::key::PublicKey,
    ) -> Result<bool> {
        Ok(true)
    }
}
