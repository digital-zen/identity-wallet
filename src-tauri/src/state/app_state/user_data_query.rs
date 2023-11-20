use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum QueryTarget {
    Credentials,
    Connections,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
pub enum SortMethod {
    #[default]
    NameAZ,
    IssuanceNewOld,
    AddedNewOld,
    FirstInteractedNewOld,
    LastInteractedNewOld,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub struct UserDataQuery {
    pub target: QueryTarget,
    #[serde(default)]
    pub search_term: Option<String>,
    #[serde(default)]
    pub sort_method: Option<SortMethod>,
    #[serde(default)]
    pub sort_reverse: Option<bool>,
}
