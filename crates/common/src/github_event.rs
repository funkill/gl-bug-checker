use std::path::Path;

use json::{object::Object, JsonValue};

const GITHUB_EVENT_PATH_VAR: &str = "GITHUB_EVENT_PATH";

type EventContent = Object;

pub struct GithubEvent(EventContent);

impl GithubEvent {
    pub fn with_content<S: AsRef<str>>(content: &S) -> crate::Result<GithubEvent> {
        let content = json::parse(content.as_ref())?;

        match content {
            JsonValue::Object(object) => Ok(GithubEvent(object)),
            _ => Err(anyhow::format_err!("Event not object")),
        }
    }

    pub fn new<P: AsRef<Path>>(event_path: P) -> crate::Result<GithubEvent> {
        let content = std::fs::read_to_string(event_path)?;
        GithubEvent::with_content(&content)
    }

    pub fn default() -> crate::Result<GithubEvent> {
        std::env::var(GITHUB_EVENT_PATH_VAR)
            .map_err(|_| anyhow::format_err!("Could not get var `{}`", GITHUB_EVENT_PATH_VAR))
            .and_then(GithubEvent::new)
    }

    pub fn pr_number(&self) -> crate::Result<u64> {
        self.0
            .get("number")
            .and_then(JsonValue::as_u64)
            .ok_or_else(|| anyhow::format_err!("Can't get PR number from event"))
    }

    pub fn base_sha(&self) -> crate::Result<String> {
        self.take_sha_for("base")
    }

    pub fn head_sha(&self) -> crate::Result<String> {
        self.take_sha_for("head")
    }

    fn take_sha_for(&self, field: &str) -> crate::Result<String> {
        self.0
            .get("pull_request")
            .ok_or_else(|| anyhow::format_err!("pull_request does not exist"))
            .and_then(|pr| match pr {
                JsonValue::Object(pr) => Ok(pr),
                _ => Err(anyhow::format_err!("pull_request not json object")),
            })
            .and_then(|pr| {
                pr.get(field)
                    .ok_or_else(|| anyhow::format_err!("{} does not exist", field))
            })
            .and_then(|head| match head {
                JsonValue::Object(head) => Ok(head),
                _ => Err(anyhow::format_err!("head not object")),
            })
            .and_then(|head| {
                head.get("sha")
                    .ok_or_else(|| anyhow::format_err!("sha does not exist"))
            })
            .and_then(|reference| match reference {
                JsonValue::String(reference) => Ok(reference.clone()),
                JsonValue::Short(reference) => Ok(reference.as_str().into()),
                _ => Err(anyhow::format_err!("sha not string")),
            })
    }
}
