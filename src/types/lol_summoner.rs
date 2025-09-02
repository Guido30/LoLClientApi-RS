use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolSummonerAccountIdAndSummonerId {
    pub account_id: u64,
    pub summoner_id: u64,
}
