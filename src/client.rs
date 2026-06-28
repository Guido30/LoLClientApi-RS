use reqwest::Client;

use crate::shared::{LeagueClientConfig, LeagueClientError};
use crate::types::{lol_champ_select, lol_champions, lol_chat, lol_heartbeat, lol_summoner};

#[derive(Debug, Clone, Default)]
pub struct LeagueClient {
    pub config: LeagueClientConfig,
    client: Client,
}

impl LeagueClient {
    pub fn new() -> Self {
        let mut client = Self::default();

        if let Ok(c) = client.build_client() {
            client.client = c;
        }

        client
    }

    fn build_client(&mut self) -> Result<Client, LeagueClientError> {
        self.config = LeagueClientConfig::new();
        let headers = self.config.build_client_headers();

        Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| e.into())
    }

    pub async fn status(&self) -> bool {
        let url = self.config.build_url("help");
        self.client.get(url).send().await.is_ok()
    }

    pub async fn retry(&mut self) -> bool {
        if self.status().await {
            return true;
        }

        if let Ok(c) = self.build_client() {
            self.client = c;
            return self.status().await;
        }

        false
    }

    pub async fn get_lol_champ_select_v1_pickable_skin_ids(
        &self,
    ) -> Result<Vec<i32>, LeagueClientError> {
        let url = self
            .config
            .build_url("lol-champ-select/v1/pickable-skin-ids");

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_champ_select_v1_current_champion(&self) -> Result<i32, LeagueClientError> {
        let url = self
            .config
            .build_url("lol-champ-select/v1/current-champion");

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_champ_select_v1_session(
        &self,
    ) -> Result<lol_champ_select::TeamBuilderDirectChampSelectSession, LeagueClientError> {
        let url = self.config.build_url("lol-champ-select/v1/session");

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn patch_lol_champ_select_v1_session_my_selection(
        &self,
        selected_skin_id: Option<i32>,
        spell1_id: Option<u64>,
        spell2_id: Option<u64>,
    ) -> Result<(), LeagueClientError> {
        let url = self
            .config
            .build_url("lol-champ-select/v1/session/my-selection");

        let body = lol_champ_select::TeamBuilderDirectChampSelectMySelection {
            selected_skin_id,
            spell1_id,
            spell2_id,
        };

        self.client.patch(url).json(&body).send().await?;

        Ok(())
    }

    pub async fn get_lol_lobby_team_builder_champ_select_v1_session(
        &self,
    ) -> Result<lol_champ_select::TeamBuilderDirectChampSelectSession, LeagueClientError> {
        let url = self
            .config
            .build_url("lol-lobby-team-builder/champ-select/v1/session");

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins(
        &self,
        summoner_id: i64,
        champion_id: i64,
    ) -> Result<Vec<lol_champions::LolChampionsCollectionsChampionSkin>, LeagueClientError> {
        let url = self.config.build_url(&format!(
            "lol-champions/v1/inventories/{}/champions/{}/skins",
            summoner_id, champion_id
        ));

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_summoner_v1_current_summoner(
        &self,
    ) -> Result<lol_summoner::LolSummonerSummoner, LeagueClientError> {
        let url = self.config.build_url("lol-summoner/v1/current-summoner");

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_summoner_v1_current_summoner_account_and_summoner_ids(
        &self,
    ) -> Result<lol_summoner::LolSummonerAccountIdAndSummonerId, LeagueClientError> {
        let url = self
            .config
            .build_url("lol-summoner/v1/current-summoner/account-and-summoner-ids");

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_summoner_v2_summoners(
        &self,
        ids: &[u64],
    ) -> Result<Vec<lol_summoner::LolSummonerSummoner>, LeagueClientError> {
        let url = self.config.build_url("lol-summoner/v2/summoners");

        let response = self
            .client
            .get(url)
            .query(&[("ids", &serde_json::to_string(ids)?)])
            .send()
            .await?;

        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_summoner_v2_summoner_names_by_puuids(
        &self,
        puuids: &[&str],
    ) -> Result<Vec<lol_summoner::LolSummonerSummonerIdAndName>, LeagueClientError> {
        let url = self
            .config
            .build_url("lol-summoner/v2/summoner-names-by-puuids");

        let response = self
            .client
            .get(url)
            .query(&[("puuids", &serde_json::to_string(&puuids)?)])
            .send()
            .await?;

        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_summoner_v2_summoners_puuid_puuid(
        &self,
        puuid: &str,
    ) -> Result<lol_summoner::LolSummonerSummoner, LeagueClientError> {
        let url = self
            .config
            .build_url(&format!("/lol-summoner/v2/summoners/puuid/{puuid}"));

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_chat_v1_friends(
        &self,
    ) -> Result<Vec<lol_chat::LolChatFriendResource>, LeagueClientError> {
        let url = self.config.build_url("lol-chat/v1/friends");

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn get_lol_chat_v1_me(
        &self,
    ) -> Result<lol_chat::LolChatUserResource, LeagueClientError> {
        let url = self.config.build_url("lol-chat/v1/me");

        let response = self.client.get(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }

    pub async fn post_lol_gameflow_v1_tick(&self) -> Result<(), LeagueClientError> {
        let url = self.config.build_url("lol-gameflow/v1/tick");

        self.client.post(url).send().await?;

        Ok(())
    }

    pub async fn post_lol_heartbeat_v1_connection_status(
        &self,
    ) -> Result<lol_heartbeat::LolHeartbeatLcdsConnection, LeagueClientError> {
        let url = self.config.build_url("lol-heartbeat/v1/connection-status");

        let response = self.client.post(url).send().await?;
        let text = response.text().await.unwrap_or_default();

        serde_json::from_str(&text).map_err(|e| LeagueClientError {
            error: Box::new(e),
            response_text: Some(text),
        })
    }
}
