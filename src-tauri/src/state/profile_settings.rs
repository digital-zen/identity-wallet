use super::SortMethod;
use crate::AppState;

pub async fn sort_credentials(state: &AppState) {
    let creds_sort_setting = state
        .active_profile
        .lock()
        .unwrap()
        .clone()
        .unwrap()
        .settings
        .credential_sort
        .clone();
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
    let connects_sort_setting = state
        .active_profile
        .lock()
        .unwrap()
        .clone()
        .unwrap()
        .settings
        .connection_sort
        .clone();
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
