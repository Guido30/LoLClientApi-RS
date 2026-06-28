use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolSummonerSummoner {
    pub summoner_id: u64,
    pub account_id: u64,
    pub display_name: String,
    pub internal_name: String,
    pub profile_icon_id: i32,
    pub summoner_level: u32,
    pub xp_since_last_level: u64,
    pub xp_until_next_level: u64,
    pub percent_complete_for_next_level: u32,
    pub reroll_points: LolSummonerSummonerRerollPoints,
    pub puuid: String,
    pub name_change_flag: bool,
    pub unnamed: bool,
    pub privacy: LolSummonerProfilePrivacySetting,
    pub game_name: String,
    pub tag_line: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolSummonerAccountIdAndSummonerId {
    pub account_id: u64,
    pub summoner_id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolSummonerSummonerIdAndName {
    pub summoner_id: u64,
    pub display_name: String,
    pub puuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolSummonerSummonerRerollPoints {
    pub points_to_reroll: u32,
    pub current_points: u32,
    pub number_of_rolls: u32,
    pub max_rolls: u32,
    pub points_cost_to_roll: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolSummonerProfilePrivacySetting {
    #[default]
    Public,
    Private,
}