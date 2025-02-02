pub mod actions;
pub mod persistence;
pub mod reducers;
pub mod user_prompt;

use self::reducers::authorization::ConnectionRequest;
use crate::{
    crypto::stronghold::StrongholdManager, state::user_prompt::CurrentUserPrompt,
    verifiable_credential_record::DisplayCredential,
};
use derivative::Derivative;
use oid4vc::oid4vc_core::Subject;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc};
use strum::EnumString;
use ts_rs::TS;

pub struct IdentityManager {
    pub subject: Arc<dyn Subject>,
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}

#[derive(Default)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
}

/// The inner state of the application managed by Tauri. When the state is serialized in order to be sent to the
/// frontend, the `managers` and `active_connection_request` fields are skipped.
#[derive(Default, Serialize, Deserialize, Derivative, TS)]
#[derivative(Debug)]
#[ts(export)]
#[serde(default)]
pub struct AppState {
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub managers: Arc<tauri::async_runtime::Mutex<Managers>>,
    pub active_profile: Option<Profile>,
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub active_connection_request: Option<ConnectionRequest>,
    pub locale: Locale,
    pub credentials: Vec<DisplayCredential>,
    pub current_user_prompt: Option<CurrentUserPrompt>,
    pub dev_mode: DevMode,
    #[ts(type = "Array<string>")]
    pub debug_messages: VecDeque<String>,
    #[ts(type = "object | null")]
    pub user_journey: Option<serde_json::Value>,
    pub connections: Vec<Connection>,
    pub user_data_query: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq, Default)]
#[ts(export, export_to = "bindings/DevMode.ts")]
pub enum DevMode {
    On,
    #[default]
    Off,
    OnWithAutologin,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            managers: self.managers.clone(),
            active_profile: self.active_profile.clone(),
            active_connection_request: serde_json::from_value(serde_json::json!(self.active_connection_request))
                .unwrap(),
            locale: self.locale.clone(),
            credentials: self.credentials.clone(),
            current_user_prompt: self.current_user_prompt.clone(),
            debug_messages: self.debug_messages.clone(),
            user_journey: self.user_journey.clone(),
            connections: self.connections.clone(),
            user_data_query: self.user_data_query.clone(),
            dev_mode: self.dev_mode.clone(),
        }
    }
}

#[derive(Default)]
pub struct AppStateContainer(pub tokio::sync::Mutex<AppState>);

/// Format of a locale string: `ll_CC` - where ll is the language code (ISO 639) and CC is the country code (ISO 3166).
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default, EnumString)]
#[ts(export)]
#[allow(non_camel_case_types)]
pub enum Locale {
    #[default]
    #[serde(rename = "en-US")]
    en_US,
    #[serde(rename = "en-GB")]
    en_GB,
    #[serde(rename = "de-DE")]
    de_DE,
    #[serde(rename = "nl-NL")]
    nl_NL,
}

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub picture: Option<String>,
    pub theme: Option<String>,
    pub primary_did: String,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Connection {
    pub id: String,
    pub client_name: String,
    pub url: String,
    pub verified: bool,
    pub first_interacted: String,
    pub last_interacted: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_app_state_serialize() {
        let state = AppState {
            active_profile: Some(Profile {
                name: "John Doe".to_string(),
                picture: None,
                theme: None,
                primary_did: "did:example:123".to_string(),
            }),
            locale: Locale::en_US,
            credentials: vec![],
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            debug_messages: Default::default(),
            user_journey: None,
            connections: vec![],
            ..Default::default()
        };

        let serialized = serde_json::to_string_pretty(&state).unwrap();

        // AppState is serialized without the `managers` and `active_connection_request` fields.
        // Probably a basic json file instead of the indoc! is cleaner.
        assert_eq!(
            serialized,
            indoc! {
            r#"{
                  "active_profile": {
                    "name": "John Doe",
                    "picture": null,
                    "theme": null,
                    "primary_did": "did:example:123"
                  },
                  "locale": "en-US",
                  "credentials": [],
                  "current_user_prompt": {
                    "type": "redirect",
                    "target": "me"
                  },
                  "dev_mode": "Off",
                  "debug_messages": [],
                  "user_journey": null,
                  "connections": [],
                  "user_data_query": []
                }"#}
        );
    }
}
