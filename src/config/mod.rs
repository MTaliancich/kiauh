use crate::config::fluidd::FluiddConfig;
use crate::config::kiauh::KIAUHConfig;
use crate::config::klipper::KlipperConfig;
use crate::config::mainsail::MainsailConfig;
use crate::config::moonraker::MoonrakerConfig;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

mod fluidd;
mod kiauh;
mod klipper;
mod mainsail;
mod moonraker;
mod repo;

#[derive(Default)]
pub(crate) struct Config {
    kiauh_config: KIAUHConfig,
    klipper_config: KlipperConfig,
    moonraker_config: MoonrakerConfig,
    mainsail_config: MainsailConfig,
    fluidd_config: FluiddConfig,
}

impl Config {
    pub(crate) fn get_kiauh(&self) -> &KIAUHConfig {
        &self.kiauh_config
    }
    pub(crate) fn get_mut_kiauh(&mut self) -> &mut KIAUHConfig {
        &mut self.kiauh_config
    }
    pub(crate) fn get_klipper(&self) -> &KlipperConfig {
        &self.klipper_config
    }
    pub(crate) fn get_mut_klipper(&mut self) -> &mut KlipperConfig {
        &mut self.klipper_config
    }
    pub(crate) fn get_moonraker(&self) -> &MoonrakerConfig {
        &self.moonraker_config
    }
    pub(crate) fn get_mut_moonraker(&mut self) -> &mut MoonrakerConfig {
        &mut self.moonraker_config
    }
    pub(crate) fn get_mainsail(&self) -> &MainsailConfig {
        &self.mainsail_config
    }
    pub(crate) fn get_mut_mainsail(&mut self) -> &mut MainsailConfig {
        &mut self.mainsail_config
    }
    pub(crate) fn get_fluidd(&self) -> &FluiddConfig {
        &self.fluidd_config
    }
    pub(crate) fn get_mut_fluidd(&mut self) -> &mut FluiddConfig {
        &mut self.fluidd_config
    }

    pub(crate) async fn save<T: AsyncWrite + Unpin>(&self, buffer: &mut T) -> std::io::Result<()> {
        let new_line = "\n".as_bytes();
        self.kiauh_config.save(buffer).await?;
        buffer.write_all(new_line).await?;
        self.klipper_config.save(buffer).await?;
        buffer.write_all(new_line).await?;
        self.moonraker_config.save(buffer).await?;
        buffer.write_all(new_line).await?;
        self.mainsail_config.save(buffer).await?;
        buffer.write_all(new_line).await?;
        self.fluidd_config.save(buffer).await
    }

    pub(crate) async fn load<T: AsyncRead + Unpin>(buffer: &mut T) -> std::io::Result<Self> {
        let mut buf = String::new();
        buffer.read_to_string(&mut buf).await?;
        let mut lines = buf
            .lines()
            .map(|s| s.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        Ok(Self {
            kiauh_config: KIAUHConfig::load(&mut lines).await,
            klipper_config: KlipperConfig::load(&mut lines).await,
            moonraker_config: MoonrakerConfig::load(&mut lines).await,
            mainsail_config: MainsailConfig::load(&mut lines).await,
            fluidd_config: FluiddConfig::load(&mut lines).await,
        })
    }
}