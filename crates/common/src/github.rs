use hubcaps::{Credentials, Github as HCGithub, comments::CommentOptions, repositories::Repository};
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
        let github_ref = std::env::var("GITHUB_REF")?;
        github_ref
            .split('/')
            .nth(2)
            .expect(&format!(
                "github_ref has unknown format: {}. Must be 'refs/pull/:prNumber/merge'",
                github_ref
            ))
            .parse()
            .map_err(Into::into)
    }

    pub async fn comment_pr(&self, text: String) -> crate::Result<()> {
        let pr_number = Github::pr_number()?;
        let comment = CommentOptions { body: text };

        self.repo.pulls().get(pr_number).comments().create(&comment).await?;

        Ok(())
    }
}
