use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnNull, serde_as};
use std::collections::HashMap;

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChatFriendResource {
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub game_name: String,
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    pub status_message: String,
    pub note: String,
    #[serde_as(as = "DefaultOnNull")]
    pub last_seen_online_timestamp: String,
    pub is_p2p_conversation_muted: bool,
    pub group_id: u32,
    pub display_group_id: u32,
    pub group_name: String,
    pub display_group_name: String,
    pub lol: HashMap<String, String>,
}
