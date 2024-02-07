use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub enum EventType {
    #[serde(rename = "initial_connection")]
    InitialConnection,
    #[serde(rename = "credential_offer")]
    CredentialOffer,
    #[serde(rename = "login")]
    Login,
}

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct HistoryEvent {
    pub title: String,
    pub sub_title: String,
    pub date: DateTime<Utc>,
    pub event_type: EventType,
    pub image_id: String,
}

pub async fn fetch_history(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    let mut history = Vec::new();

    for conn in state.connections.iter() {
        history.push(HistoryEvent {
            title: conn.client_name.to_string(),
            sub_title: conn.url.to_string(),
            date: conn.first_interacted.clone(),
            event_type: EventType::InitialConnection,
            image_id: conn.id.clone(),
        });
    }

    for cred in state.credentials.iter() {
        history.push(HistoryEvent {
            title: cred.metadata.display.name.clone().unwrap_or("test".to_string()),
            sub_title: "Some sub title".to_string(),
            date: cred.metadata.date_added,
            image_id: cred.metadata.display.icon.clone().unwrap_or("".to_string()),
            event_type: EventType::CredentialOffer,
        });
    }

    state.history = history;

    Ok(state)
}
