use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnNull, serde_as};

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionSkin {
    pub champion_id: i32,
    #[serde_as(as = "DefaultOnNull")]
    pub chroma_path: String,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub last_selected: bool,
    pub skin_augments: LolChampionsCollectionsChampionSkinAugments,
    pub splash_path: String,
    pub tile_path: String,
    pub chromas: Vec<LolChampionsCollectionsChampionChroma>,
    pub quest_skin_info: LolChampionsChampionQuestSkinInfo,
    pub emblems: Vec<LolChampionsCollectionsChampionSkinEmblem>,
    pub uncentered_splash_path: String,
    pub load_screen_path: String,
    pub rarity_gem_path: String,
    #[serde_as(as = "DefaultOnNull")]
    pub splash_video_path: String,
    #[serde_as(as = "DefaultOnNull")]
    pub collection_splash_video_path: String,
    pub skin_type: String,
    #[serde_as(as = "DefaultOnNull")]
    pub features_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
    pub rental: LolChampionsCollectionsRental,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsRental {
    pub end_date: u64,
    pub purchase_date: u64,
    pub win_count_remaining: i32,
    pub rented: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionSkinAugments {
    pub augments: Vec<LolChampionsCollectionsChampionSkinAugment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionSkinAugment {
    pub content_id: String,
    pub overlays: Vec<LolChampionsCollectionsChampionSkinAugmentOverlays>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionSkinAugmentOverlays {
    pub centered_l_c_overlay_path: String,
    pub social_card_l_c_overlay_path: String,
    pub tile_l_c_overlay_path: String,
    pub uncentered_l_c_overlay_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionChroma {
    pub champion_id: i32,
    pub chroma_path: String,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub last_selected: bool,
    pub skin_augments: LolChampionsCollectionsChampionSkinAugments,
    pub colors: Vec<String>,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsChampionQuestSkinInfo {
    pub name: String,
    pub description_info: Vec<LolChampionsQuestSkinDescriptionInfo>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub collection_description: String,
    pub tiers: Vec<LolChampionsCollectionsChampionQuestSkin>,
    #[serde_as(as = "DefaultOnNull")]
    pub product_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsQuestSkinDescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionQuestSkin {
    pub champion_id: i32,
    pub chroma_path: String,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub last_selected: bool,
    pub skin_augments: LolChampionsCollectionsChampionSkinAugments,
    pub splash_path: String,
    pub tile_path: String,
    pub stage: u64,
    pub description: String,
    pub uncentered_splash_path: String,
    pub load_screen_path: String,
    pub short_name: String,
    pub splash_video_path: String,
    pub collection_splash_video_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionSkinEmblem {
    pub name: String,
    pub emblem_path: LolChampionsCollectionsChampionSkinEmblemPath,
    pub positions: LolChampionsCollectionsChampionSkinEmblemPosition,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LolChampionsCollectionsChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}
