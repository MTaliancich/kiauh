use tokio::io::{AsyncWrite, AsyncWriteExt};

pub(crate) struct Repository {
    repo_url: String,
    branch: String,
}

impl Repository {
    pub(crate) fn new(repo_url: &str, branch: &str) -> Self {
        Self {
            repo_url: repo_url.to_string(),
            branch: branch.to_string(),
        }
    }

    pub(crate) fn new_repo(repo_url: &str) -> Self {
        Self {
            repo_url: repo_url.to_string(),
            branch: "main".to_string(),
        }
    }

    pub(crate) fn set_branch(&mut self, branch: &str) {
        self.branch = branch.to_string();
    }

    pub(crate) fn set_repo_url(&mut self, repo_url: &str) {
        self.repo_url = repo_url.to_string();
    }

    pub(crate) fn repo_url(&self) -> &str {
        &self.repo_url
    }

    pub(crate) fn branch(&self) -> &str {
        &self.branch
    }

    pub(crate) async fn save<T: AsyncWrite + Unpin>(&self, buffer: &mut T) -> std::io::Result<()> {
        buffer.write_all("repo_url: ".as_bytes()).await?;
        buffer.write_all(self.repo_url.as_bytes()).await?;
        buffer.write_all("\nbranch: ".as_bytes()).await?;
        buffer.write_all(self.branch.as_bytes()).await?;
        buffer.write_all("\n".as_bytes()).await
    }

    pub(crate) async fn load(&mut self, buffer: &[&str]) {
        for line in buffer {
            if let Some((key, value)) = line.split_once(':') {
                match key.trim() {
                    "repo_url" => self.repo_url = value.trim().to_string(),
                    "branch" => self.branch = value.trim().to_string(),
                    &_ => {}
                }
            }
        }
    }
}