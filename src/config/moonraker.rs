use crate::config::repo::Repository;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub(crate) struct MoonrakerConfig {
    repo: Repository,
}

impl Default for MoonrakerConfig {
    fn default() -> Self {
        Self {
            repo: Repository::new("https://github.com/Arksine/moonraker", "master"),
        }
    }
}

impl MoonrakerConfig {
    pub(crate) fn get_repo(&self) -> &Repository {
        &self.repo
    }

    pub(crate) fn get_mut_repo(&mut self) -> &mut Repository {
        &mut self.repo
    }

    pub(crate) async fn save<T: AsyncWrite + Unpin>(&self, buffer: &mut T) -> std::io::Result<()> {
        buffer.write_all("[moonraker]\n".as_bytes()).await?;
        self.repo.save(buffer).await
    }

    pub(crate) async fn load(buffer: &mut Vec<&str>) -> Self {
        let mut tmp = Self::default();
        let index = buffer.iter().position(|x| x == &"[moonraker]");
        if let Some(index) = index {
            let mut end = index + 1;
            while let Some(line) = buffer.get(end) {
                if !line.contains(':') {
                    break;
                }

                end += 1
            }

            tmp.repo.load(&buffer[index..end]).await;

            for i in (index..end).rev() {
                buffer.remove(i);
            }
        }
        tmp
    }
}