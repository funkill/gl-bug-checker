use crate::{github_event::GithubEvent, git::Git, github::Github};

pub struct Project {
    git: Git,
    github: Github,
    event: GithubEvent,
}

impl Project {
    pub fn new<T: Into<String>>(token: T) -> crate::Result<Project> {
        let git = Git::in_current_dir()?;
        let github = Github::new(token)?;
        let event = GithubEvent::default()?;

        Ok(Project { git, github, event })
    }

    pub fn changed_files(&self) -> crate::Result<Vec<String>> {
        let head = self.event.head_sha()?;
        let base = self.event.base_sha()?;

        self.git.updated_files(&base, &head)
    }

    pub async fn comment_pr(&self, text: String) -> crate::Result<()> {
        self.github.comment_pr(self.event.pr_number()?, text).await
    }
}
