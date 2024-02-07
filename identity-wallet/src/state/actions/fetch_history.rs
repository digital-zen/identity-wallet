use crate::{reducer, state::reducers::{history::fetch_history, Reducer}};

use super::ActionTrait;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FetchHistory;

#[typetag::serde(name = "[History] Fetch")]
impl ActionTrait for FetchHistory {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(fetch_history)]
    }
}
