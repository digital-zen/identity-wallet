use log::debug;
use tokio::{
    fs::{read, remove_file, File},
    io::AsyncWriteExt,
};

use crate::state::app_state::AppState;
use crate::state::reducers::profile_settings::sort_connections;
use crate::STATE_FILE;

/// Loads an [AppState] from the app's data directory.
/// If it does not exist or it cannot be parsed, it will fallback to default values.
pub async fn load_state() -> anyhow::Result<AppState> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let bytes = read(state_file).await?;
    let content = String::from_utf8(bytes)?;
    let app_state: AppState = serde_json::from_str(&content)?;

    // Sorts connections according to profile preference.
    sort_connections(&app_state).await;

    debug!("state loaded from disk");
    Ok(app_state)
}

/// Persists a [AppState] to the app's data directory.
pub async fn save_state(app_state: &AppState) -> anyhow::Result<()> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let mut file = File::create(state_file).await?;
    
    // Turn app_state into json_value and remove sensitive information (credentials).
    let mut data = serde_json::to_value(app_state).unwrap();
    let credentials = data.get_mut("credentials").unwrap();
    *credentials = serde_json::Value::Null;

    file.write_all(data.to_string().as_bytes()).await?;
    debug!("state saved to disk");
    Ok(())
}

// Removes the state file from the app's data directory.
pub async fn delete_state_file() -> anyhow::Result<()> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    remove_file(state_file).await?;
    debug!("state deleted from disk");
    Ok(())
}

pub async fn delete_stronghold() -> anyhow::Result<()> {
    let stronghold_file = crate::STRONGHOLD.lock().unwrap().clone();
    remove_file(&stronghold_file).await?;
    remove_file(stronghold_file.join(".snapshot")).await?;
    debug!("stronghold deleted from disk");
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_load_state() {
        // TODO: how to mock the app_handle?
    }
}
