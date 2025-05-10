use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct CratesResponse {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
}

#[derive(Debug, Deserialize)]
struct CrateInfo {
    #[serde(rename = "max_version")]
    latest_version: String,
}

pub struct UpdateChecker {
    current_version: String,
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            current_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn check_for_updates(&self) -> Result<(), Box<dyn Error>> {
        match self.get_latest_version() {
            Ok(latest_version) => {
                if latest_version != self.current_version {
                    println!(
                        "¡Nueva versión {} disponible! Ejecuta 'cargo install faspi' para actualizar.",
                        latest_version
                    );
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Error al verificar actualizaciones: {}", e);
                Ok(())
            }
        }
    }

    fn get_latest_version(&self) -> Result<String, Box<dyn Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;

        let response = client
            .get("https://crates.io/api/v1/crates/faspi")
            .header("User-Agent", "faspi-cli")
            .send()?;

        let data: CratesResponse = response.json()?;
        Ok(data.crate_info.latest_version)
    }
}