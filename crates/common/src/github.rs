use hubcaps::{
    comments::CommentOptions, repositories::Repository, Credentials, Github as HCGithub,
};

const DEFAULT_USER_AGENT: &str = "rust-lang-ru-bot";
const REPO_NAME_VAR: &str = "GITHUB_REPOSITORY";

pub struct Github {
    repo: Repository,
}

impl Github {
    pub fn new<S: Into<String>>(token: S) -> crate::Result<Github> {
        let repo_name = std::env::var(REPO_NAME_VAR)
            .map_err(|_| anyhow::format_err!("Could not get var `{}`", REPO_NAME_VAR))?;
        let (owner, repo) = repo_name.split_once('/').ok_or_else(|| {
            anyhow::format_err!(
                "Unknown format of repo: {}. Must by 'owner/repo'",
                repo_name
            )
        })?;

        let github = HCGithub::new(
            String::from(DEFAULT_USER_AGENT),
            Credentials::Token(token.into()),
        )?;
        let repo = github.repo(owner, repo);

        Ok(Github { repo })
    }

    pub async fn comment_pr(&self, pr_number: u64, text: String) -> crate::Result<()> {
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
