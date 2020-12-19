use hubcaps::{
    comments::CommentOptions, repositories::Repository, Credentials, Github as HCGithub,
};
pub struct Github {
    repo: Repository,
}

impl Github {
    pub fn new(token: String) -> crate::Result<Github> {
        let github = HCGithub::new("rust-lang-ru-bot", Credentials::Token(token))?;
        let repo_var = std::env::var("GITHUB_REPOSITORY")?;
        let (owner, repo) = repo_var.split_once('/').expect(&format!(
            "Unknown format of repo: {}. Must by 'owner/repo'",
            repo_var
        ));
        let repo = github.repo(owner, repo);

        Ok(Github { repo })
    }

    pub fn pr_number() -> crate::Result<u64> {
        use json::JsonValue;

        let event_path = std::env::var("GITHUB_EVENT_PATH")?;
        let event = std::fs::read_to_string(event_path)?;
        let value = json::parse(&event)?;

        match value {
            JsonValue::Object(value) => value
                .get("number")
                .and_then(JsonValue::as_u64)
                .ok_or_else(|| anyhow::format_err!("Can't get PR number from event")),
            _ => Err(anyhow::format_err!("Event not object")),
        }
    }

    pub async fn comment_pr(&self, text: String) -> crate::Result<()> {
        let pr_number = Github::pr_number()?;
        let comment = CommentOptions { body: text };

        self.repo
            .pulls()
            .get(pr_number)
            .comments()
            .create(&comment)
            .await?;

        Ok(())
    }
}
