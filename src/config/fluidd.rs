use tokio::io::{AsyncWrite, AsyncWriteExt};

pub(crate) struct FluiddConfig {
    port: u16,
    unstable_releases: bool,
}

impl Default for FluiddConfig {
    fn default() -> Self {
        Self {
            port: 80,
            unstable_releases: false,
        }
    }
}

impl FluiddConfig {
    pub(crate) fn port(&self) -> u16 {
        self.port
    }
    pub(crate) fn unstable_releases(&self) -> bool {
        self.unstable_releases
    }
    pub(crate) fn set_unstable_releases(&mut self) {
        self.unstable_releases = true;
    }
    pub(crate) fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub(crate) async fn save<T: AsyncWrite + Unpin>(&self, buffer: &mut T) -> std::io::Result<()> {
        buffer.write_all("[fluidd]\nport: ".as_bytes()).await?;
        buffer.write_all(self.port.to_string().as_bytes()).await?;
        if self.unstable_releases {
            buffer
                .write_all("\nunstable_releases: True\n".as_bytes())
                .await
        } else {
            buffer
                .write_all("\nunstable_releases: False\n".as_bytes())
                .await
        }
    }

    pub(crate) async fn load(buffer: &mut Vec<&str>) -> Self {
        let mut tmp = Self::default();
        let index = buffer.iter().position(|x| x == &"[fluidd]");
        if let Some(index) = index {
            let mut end = index + 1;
            while let Some(line) = buffer.get(end) {
                if let Some((key, value)) = line.split_once(':') {
                    match key.trim() {
                        "port" => tmp.port = value.parse::<u16>().unwrap_or(tmp.port),
                        "unstable_releases" => tmp.unstable_releases = value.trim() == "True",
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