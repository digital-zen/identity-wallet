use crate::{
    crypto::stronghold::StrongholdManager, state::user_prompt::CurrentUserPrompt,
    verifiable_credential_record::DisplayCredential,
};
use derivative::Derivative;
use oid4vc_core::Subject;
use oid4vc_manager::ProviderManager;
use oid4vci::Wallet;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use ts_rs::TS;

use self::reducers::authorization::ConnectionRequest;

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
