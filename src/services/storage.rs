use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::domain::AppState;

const APP_STATE_PATH: &str = "data/app_state.json";

#[derive(Debug, Default)]
pub struct JsonStorage;

impl JsonStorage {
    pub fn load_app_state(&self) -> Result<AppState> {
        load_app_state()
    }

    pub fn save_app_state(&self, state: &AppState) -> Result<()> {
        save_app_state(state)
    }
}

pub fn load_app_state() -> Result<AppState> {
    let path = Path::new(APP_STATE_PATH);

    if !path.exists() {
        let default_state = AppState::default();
        save_app_state(&default_state)?;
        return Ok(default_state);
    }

    let raw = fs::read_to_string(path)
        .with_context(|| format!("Nem sikerult beolvasni: {}", path.display()))?;

    let parsed: AppState =
        serde_json::from_str(&raw).with_context(|| "Hibas JSON allapotfajl formatum")?;

    Ok(parsed)
}

pub fn save_app_state(state: &AppState) -> Result<()> {
    let path = Path::new(APP_STATE_PATH);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Nem sikerult letrehozni az allapotfajl konyvtarat: {}",
                parent.display()
            )
        })?;
    }

    let tmp_path = path.with_extension("json.tmp");
    let payload = serde_json::to_string_pretty(state).context("Nem sikerult JSON-t generalni")?;

    fs::write(&tmp_path, payload)
        .with_context(|| format!("Nem sikerult ideiglenes fajlba irni: {}", tmp_path.display()))?;

    if path.exists() {
        fs::remove_file(path)
            .with_context(|| format!("Nem sikerult torolni a regi fajlt: {}", path.display()))?;
    }

    fs::rename(&tmp_path, path).with_context(|| {
        format!(
            "Nem sikerult az ideiglenes fajlt atnevezni veglegesre: {}",
            path.display()
        )
    })?;

    Ok(())
}
