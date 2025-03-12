use tokio::io::{AsyncWrite, AsyncWriteExt};

#[derive(Default)]
pub(crate) struct KIAUHConfig {
    backup_before_update: bool,
}

impl KIAUHConfig {
    pub(crate) fn get_backup_before_update(&self) -> bool {
        self.backup_before_update
    }

    pub(crate) fn set_backup_before_update(&mut self, backup_before_update: bool) {
        self.backup_before_update = backup_before_update;
    }

    pub(crate) async fn save<T: AsyncWrite + Unpin>(&self, buffer: &mut T) -> std::io::Result<()> {
        buffer
            .write_all("[kiauh]\nbackup_before_update: ".as_bytes())
            .await?;
        if self.backup_before_update {
            buffer.write_all("True\n".as_bytes()).await
        } else {
            buffer.write_all("False\n".as_bytes()).await
        }
    }

    pub(crate) async fn load(buffer: &mut Vec<&str>) -> Self {
        let mut tmp = Self::default();
        let index = buffer.iter().position(|x| x == &"[kiauh]");
        if let Some(index) = index {
            let mut end = index + 1;
            while let Some(line) = buffer.get(end) {
                if let Some((key, value)) = line.split_once(':') {
                    match key.trim() {
                        "backup_before_update" => tmp.backup_before_update = value.trim() == "True",
                        &_ => {}
                    }
                } else {
                    break;
                }

                end += 1
            }

            for i in (index..end).rev() {
                buffer.remove(i);
            }
        }
        tmp
    }
}