use reqwest::blocking::Client;

use crate::shared::{LeagueClientConfig, LeagueClientError};
use crate::types::{lol_champ_select, lol_champions, lol_summoner};

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

    // Tests connection to the LCU help endpoint
    pub fn status(&self) -> bool {
        let url = self.config.build_url("help");
        self.client.get(url).send().is_ok()
    }

    // Tests if the LCU client is connected, if not tries to reconnect
    pub fn retry(&mut self) -> bool {
        if self.status() {
            return true;
        }

        if let Ok(c) = self.build_client() {
            self.client = c;
            return self.status();
        }

        false
    }

    // Establishes a new client connected to the LCU
    fn build_client(&mut self) -> Result<Client, LeagueClientError> {
        self.config = LeagueClientConfig::new();
        let headers = self.config.build_client_headers();

        Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| e.into())
    }

    pub fn get_lol_champ_select_v1_pickable_skin_ids(
        &self,
    ) -> Result<Vec<i32>, LeagueClientError> {
        let url = self
            .config
            .build_url("lol-champ-select/v1/pickable-skin-ids");
        let response = self.client.get(url).send()?;
        let text = response.text().unwrap_or_default();
        let result =
            serde_json::from_str(&text).map_err(|e| LeagueClientError {
                error: Box::new(e),
                response_text: Some(text),
            })?;
        Ok(result)
    }

    pub fn get_lol_champ_select_v1_current_champion(
        &self,
    ) -> Result<i32, LeagueClientError> {
        let url = self
            .config
            .build_url("lol-champ-select/v1/current-champion");
        let response = self.client.get(url).send()?;
        let text = response.text().unwrap_or_default();
        let result =
            serde_json::from_str(&text).map_err(|e| LeagueClientError {
                error: Box::new(e),
                response_text: Some(text),
            })?;
        Ok(result)
    }

    pub fn get_lol_summoner_v1_current_summoner_account_and_summoner_ids(
        &self,
    ) -> Result<
        lol_summoner::LolSummonerAccountIdAndSummonerId,
        LeagueClientError,
    > {
        let url = self.config.build_url(
            "lol-summoner/v1/current-summoner/account-and-summoner-ids",
        );
        let response = self.client.get(url).send()?;
        let text = response.text().unwrap_or_default();
        let result =
            serde_json::from_str(&text).map_err(|e| LeagueClientError {
                error: Box::new(e),
                response_text: Some(text),
            })?;
        Ok(result)
    }

    pub fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins(
        &self,
        summoner_id: i64,
        champion_id: i64,
    ) -> Result<
        Vec<lol_champions::LolChampionsCollectionsChampionSkin>,
        LeagueClientError,
    > {
        let url = self.config.build_url(&format!(
            "lol-champions/v1/inventories/{}/champions/{}/skins",
            summoner_id, champion_id
        ));
        let response = self.client.get(url).send()?;
        let text = response.text().unwrap_or_default();
        let result =
            serde_json::from_str(&text).map_err(|e| LeagueClientError {
                error: Box::new(e),
                response_text: Some(text),
            })?;
        Ok(result)
    }

    pub fn get_lol_champ_select_v1_session(
        &self,
    ) -> Result<
        lol_champ_select::TeamBuilderDirectChampSelectSession,
        LeagueClientError,
    > {
        let url = self.config.build_url("lol-champ-select/v1/session");
        let response = self.client.get(url).send()?;
        let text = response.text().unwrap_or_default();
        let result =
            serde_json::from_str(&text).map_err(|e| LeagueClientError {
                error: Box::new(e),
                response_text: Some(text),
            })?;
        Ok(result)
    }

    pub fn patch_lol_champ_select_v1_session_my_selection(
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

        self.client.patch(url).json(&body).send()?;
        Ok(())
    }
}
