use crate::state::{actions::Action, profile_settings};
use super::SortMethod;
use crate::AppState;

// A set_profile_locale function should be located here as well

pub async fn sort_credentials(state: &AppState) {
    let creds_sort_setting = &state
        .active_profile
        .lock()
        .unwrap()
        .clone()
        .unwrap()
        .settings
        .credential_sort;
    match creds_sort_setting {
        SortMethod::NameAZ => {
            state
                .credentials
                .lock()
                .unwrap()
                .sort_by(|a, b| a.issuer_name.cmp(&b.issuer_name));
        }
        SortMethod::IssuanceNewOld => {
            state
                .credentials
                .lock()
                .unwrap()
                .sort_by(|a, b| a.metadata.date_issued.cmp(&b.metadata.date_issued));
        }
        SortMethod::AddedNewOld => {
            state
                .credentials
                .lock()
                .unwrap()
                .sort_by(|a, b| a.metadata.date_added.cmp(&b.metadata.date_added));
        }
        _ => (),
    };
}

pub async fn sort_connections(state: &AppState) {
    let connects_sort_setting = &state
        .active_profile
        .lock()
        .unwrap()
        .clone()
        .unwrap()
        .settings
        .connection_sort;
    match connects_sort_setting {
        SortMethod::NameAZ => {
            state
                .connections
                .lock()
                .unwrap()
                .sort_by(|a, b| a.client_name.cmp(&b.client_name));
        }
        SortMethod::IssuanceNewOld => {
            state
                .connections
                .lock()
                .unwrap()
                .sort_by(|a, b| a.first_interacted.cmp(&b.first_interacted));
        }
        SortMethod::AddedNewOld => {
            state
                .connections
                .lock()
                .unwrap()
                .sort_by(|a, b| a.last_interacted.cmp(&b.last_interacted));
        }
        _ => (),
    };
}

async fn update_setting(state: &AppState, payload: serde_json::Value) {
    
}

pub async fn profile_setting_update(state: &AppState, action: Action) -> anyhow::Result<()> {
    //let setting_update = serde_json::from_value(action.payload.unwrap()).unwrap();

    match action.payload["target"] {
        "credentials_sort" => {
            update_setting(state, action.payload);
            sort_credentials(state);
        },
        "connections_sort" => {
            update_setting(state, action.payload);
            sort_connections(state)
        },
    }
}
