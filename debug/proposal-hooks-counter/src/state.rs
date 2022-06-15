use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub should_error: bool,
}
pub const CONFIG: Item<Config> = Item::new("config");
pub const VOTE_COUNTER: Item<u64> = Item::new("vote_counter");
pub const PROPOSAL_COUNTER: Item<u64> = Item::new("proposal_counter");
pub const STATUS_CHANGED_COUNTER: Item<u64> = Item::new("stauts_changed_counter");
