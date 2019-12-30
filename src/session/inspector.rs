use crate::domain::selenium::{Capabilities, Command, SessionStatus};
use crate::domain::AppState;
use bytes::Bytes;
use hyper::{Method, Request, Response,Error,service::Service};

/// Inspect the given chunk from a request's payload.
///
/// This function retrieve the method and the path from the request
/// to know which event to match.
pub fn inspect(req: Request<AppState>, chunk: Bytes) -> Bytes {
    let method = req.method().to_string();
    let path = req.uri().to_string();
    let auth_user = req.state().auth_user.clone();

    // bytes to string for deserialization
    let chunk_str = std::str::from_utf8(&chunk).unwrap_or(&"").to_owned();

    if method == "DELETE" {
        capture_delete_event(path, auth_user);
    } else if method == "POST" && is_a_new_session(&path) {
        capture_create_event(&chunk_str, auth_user);
    } else if method == "POST" && !is_a_new_session(&path) {
        capture_url_event(&chunk_str, path, auth_user);
    }

    chunk
}

/// Capture new sessions and log
fn capture_create_event(chunk: &str, auth_user: String) {
    let caps = Capabilities::deserialize(chunk).unwrap_or_else(|_| {
        error!(
            "Fail to deserialize the capabilities for the given chunk : {}",
            chunk
        );
        Capabilities::new()
    });

    let desired_caps = caps.desired_capabilities;

    // user IP/ID | session status | Platform | Browser
    info!(
        "[{}] [{}] [{}] [{}]",
        auth_user,
        SessionStatus::Creating,
        desired_caps.get_platform(),
        desired_caps.get_browser_name()
    );
}

/// Capture session deletions and log
fn capture_delete_event(path: String, auth_user: String) {
    let session_id = session_id_of_path(&path).unwrap_or_else(|| "".to_string());

    // user IP/ID | session status | session ID
    info!(
        "[{}] [{}] [{}]",
        auth_user,
        SessionStatus::Deleting,
        session_id
    );
}

/// Capture asked urls from test sessions and log
fn capture_url_event(chunk: &str, path: String, auth_user: String) {
    if path.contains("/url") {
        // deserialize the command from the request's body
        // or return a new command with an empty url
        let command = Command::deserialize(chunk).unwrap_or_else(|_| Command::new());

        let session_id = session_id_of_path(&path).unwrap_or_else(|| "".to_string());

        // user IP/ID | session_status | session ID | url_command | url
        info!(
            "[{}] [{}] [{}] [{}] [{}]",
            auth_user,
            SessionStatus::RunCommand,
            session_id,
            "url",
            command.url()
        );
    }
}

/// Split the path to determine if it's a new session
/// (the path doesn't contain the session's id) or if it's
/// an existing session (the path contains the session's id).
/// If the head and the tail of the path are empty,
/// it's a new session event that we want to capture.
fn is_a_new_session(path: &str) -> bool {
    let splitted_path: Vec<&str> = path
        .split("/wd/hub/session")
        .filter(|item| !item.is_empty())
        .collect();

    splitted_path.is_empty()
}

/// Split the given path and try to retrieve the
/// session's id.
fn session_id_of_path(path: &str) -> Option<String> {
    // Try to get the session's id part
    // e.g. possible patterns :
    // /wd/hub/session
    // /wd/hub/session/:id
    // /wd/hub/session/:id/:cmd
    let tail: Vec<&str> = path
        .split("/wd/hub/session")
        .filter(|item| !item.is_empty())
        .collect();

    // Check if there is a remainder with a session's id
    // e.g. possible patterns :
    // /:id
    // /:id/:cmd
    if !tail.is_empty() {
        let remainder: Vec<&str> = tail[0].split('/').filter(|s| !s.is_empty()).collect();

        return Some(remainder[0].to_string());
    }

    None
}
