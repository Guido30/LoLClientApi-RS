use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectChampSelectSession {
    pub id: String,
    pub game_id: u64,
    pub queue_id: i32,
    pub timer: TeamBuilderDirectTypesChampSelectTimer,
    pub chat_details: TeamBuilderDirectChampSelectChatRoomDetails,
    pub my_team: Vec<TeamBuilderDirectChampSelectPlayerSelection>,
    pub their_team: Vec<TeamBuilderDirectChampSelectPlayerSelection>,
    pub trades: Vec<TeamBuilderDirectChampSelectSwapContract>,
    pub pick_order_swaps: Vec<TeamBuilderDirectChampSelectSwapContract>,
    pub position_swaps: Vec<TeamBuilderDirectChampSelectSwapContract>,
    pub actions: Vec<serde_json::Value>,
    pub bans: TeamBuilderDirectChampSelectBannedChampions,
    pub local_player_cell_id: i64,
    pub is_spectating: bool,
    pub allow_skin_selection: bool,
    pub allow_subset_champion_picks: bool,
    pub allow_duplicate_picks: bool,
    pub allow_battle_boost: bool,
    pub boostable_skin_count: i32,
    pub allow_rerolling: bool,
    pub rerolls_remaining: u64,
    pub allow_locked_events: bool,
    pub locked_event_index: i32,
    pub bench_enabled: bool,
    pub bench_champions: Vec<TeamBuilderDirectBenchChampion>,
    pub counter: i64,
    pub skip_champion_select: bool,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
    pub show_quit_button: bool,
    pub is_legacy_champ_select: bool,
    pub is_custom_game: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectTypesChampSelectTimer {
    pub adjusted_time_left_in_phase: i64,
    pub total_time_in_phase: i64,
    pub phase: String,
    pub is_infinite: bool,
    pub internal_now_in_epoch_ms: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectChampSelectChatRoomDetails {
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub muc_jwt_dto: TeamBuilderDirectMucJwtDto,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
    pub domain: String,
    pub target_region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectChampSelectPlayerSelection {
    pub cell_id: i64,
    pub champion_id: i32,
    pub selected_skin_id: i32,
    pub ward_skin_id: i64,
    pub spell1_id: u64,
    pub spell2_id: u64,
    pub team: i32,
    pub assigned_position: String,
    pub champion_pick_intent: i32,
    pub player_type: String,
    pub summoner_id: u64,
    pub game_name: String,
    pub tag_line: String,
    pub puuid: String,
    pub is_humanoid: bool,
    pub name_visibility_type: String,
    pub player_alias: String,
    pub obfuscated_summoner_id: u64,
    pub obfuscated_puuid: String,
    pub internal_name: String,
    pub pick_mode: i32,
    pub pick_turn: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectChampSelectSwapContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: TeamBuilderDirectChampSelectSwapState,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TeamBuilderDirectChampSelectSwapState {
    Accepted,
    Cancelled,
    Declined,
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
    #[serde(other)]
    #[default]
    Unknown,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectChampSelectBannedChampions {
    pub my_team_bans: Vec<i32>,
    pub their_team_bans: Vec<i32>,
    pub num_bans: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectBenchChampion {
    pub champion_id: i32,
    pub is_priority: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamBuilderDirectChampSelectMySelection {
    pub selected_skin_id: Option<i32>,
    pub spell1_id: Option<u64>,
    pub spell2_id: Option<u64>,
}
