use crate::config::Versions;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Core {
    // Ядро
    #[serde(default)]
    pub provider: Provider,
    // Версия ядра
    #[serde(default)]
    pub version: Versions,
    // Приостановить обновление
    #[serde(default)]
    pub freeze: bool,
    // Нужно обновить
    #[serde(default)]
    pub force_update: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub enum Provider {
    #[default]
    Vanilla,
    Bucket,
    Spigot,
    Paper,
    Purpur,
    Fabric,
    Forge,
    NeoForge,
}
