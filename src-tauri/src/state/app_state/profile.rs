use serde::{Deserialize, Serialize};
use ts_rs::TS;
use super::user_data_query::SortMethod;

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub picture: Option<String>,
    pub theme: Option<String>,
    pub primary_did: String,
    pub settings: Settings,
}

/// Profile settings and preferences
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
pub struct Settings {
    pub profile_locale: Locale,
    pub credential_sort: SortMethod,
    pub connection_sort: SortMethod,
}

pub enum Setting {
    
}

/// Language setting of the entire app
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Locale {
    #[default]
    En,
    De,
    Nl,
}

pub struct SettingUpdate {
    target: String,
    update: String
}

