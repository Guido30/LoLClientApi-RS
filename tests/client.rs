use lolclientapi_rs::LeagueClient;

#[tokio::test]
async fn test_lcu_connection() {
    let client = LeagueClient::new();

    dbg!(&client.config.port);
    dbg!(&client.config.auth_token);
    dbg!(&client.config.auth_token_encoded);

    assert!(client.status().await);
}

mod lol_champ_select {
    use super::LeagueClient;

    #[tokio::test]
    async fn test_get_lol_champ_select_v1_pickable_skin_ids() {
        let client = LeagueClient::new();
        let res = client.get_lol_champ_select_v1_pickable_skin_ids().await;
        dbg!(&res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_lol_champ_select_v1_current_champion() {
        let client = LeagueClient::new();
        let res = client.get_lol_champ_select_v1_current_champion().await;
        dbg!(&res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_lol_champ_select_v1_session() {
        let client = LeagueClient::new();
        let res = client.get_lol_champ_select_v1_session().await;
        dbg!(&res);
        assert!(res.is_ok());
    }
}

mod lol_team_builder {
    use super::LeagueClient;

    #[tokio::test]
    async fn test_get_lol_lobby_team_builder_champ_select_v1_session() {
        let client = LeagueClient::new();
        let res = client
            .get_lol_lobby_team_builder_champ_select_v1_session()
            .await;

        dbg!(&res);
        assert!(res.is_ok());
    }
}

mod lol_champions {
    use super::LeagueClient;

    #[tokio::test]
    async fn test_get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins() {
        let client = LeagueClient::new();

        let summoner_id = client
            .get_lol_summoner_v1_current_summoner_account_and_summoner_ids()
            .await
            .unwrap()
            .summoner_id;

        let res = client
            .get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins(
                summoner_id as i64,
                1,
            )
            .await;

        dbg!(&res);
        assert!(res.is_ok());
    }
}

mod lol_summoner {
    use super::LeagueClient;

    #[tokio::test]
    async fn test_get_lol_summoner_v1_current_summoner() {
        let client = LeagueClient::new();
        let res = client.get_lol_summoner_v1_current_summoner().await;

        dbg!(&res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_lol_summoner_v1_current_summoner_account_and_summoner_ids() {
        let client = LeagueClient::new();
        let res = client
            .get_lol_summoner_v1_current_summoner_account_and_summoner_ids()
            .await;

        dbg!(&res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_lol_summoner_v2_summoners() {
        let client = LeagueClient::new();

        let id = client
            .get_lol_summoner_v1_current_summoner()
            .await
            .unwrap()
            .summoner_id;

        let ids = [id, id];

        let res = client.get_lol_summoner_v2_summoners(&ids).await;

        dbg!(&res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_lol_summoner_v2_summoner_names_by_puuids() {
        let client = LeagueClient::new();

        let puuid = client
            .get_lol_summoner_v1_current_summoner()
            .await
            .unwrap()
            .puuid;

        let puuids: [&str; 2] = [&puuid, &puuid];

        let res = client
            .get_lol_summoner_v2_summoner_names_by_puuids(&puuids)
            .await;

        dbg!(&res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_lol_summoner_v2_summoners_puuid_puuid() {
        let client = LeagueClient::new();

        let puuid = client
            .get_lol_summoner_v1_current_summoner()
            .await
            .unwrap()
            .puuid;

        let res = client
            .get_lol_summoner_v2_summoners_puuid_puuid(&puuid)
            .await;

        dbg!(&res);
        assert!(res.is_ok());
    }
}

mod lol_chat {
    use super::LeagueClient;

    #[tokio::test]
    async fn test_get_lol_chat_v1_friends() {
        let client = LeagueClient::new();
        let res = client.get_lol_chat_v1_friends().await;

        dbg!(&res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_lol_chat_v1_me() {
        let client = LeagueClient::new();
        let res = client.get_lol_chat_v1_me().await;

        dbg!(&res);
        assert!(res.is_ok());
    }
}

mod lol_gameflow {
    use super::LeagueClient;

    #[tokio::test]
    async fn test_post_lol_gameflow_v1_tick() {
        let client = LeagueClient::new();
        let res = client.post_lol_gameflow_v1_tick().await;

        dbg!(&res);
        assert!(res.is_ok());
    }
}

mod lol_heartbeat {
    use super::LeagueClient;

    #[tokio::test]
    async fn test_post_lol_heartbeat_v1_connection_status() {
        let client = LeagueClient::new();
        let res = client.post_lol_heartbeat_v1_connection_status().await;

        dbg!(&res);
        assert!(res.is_ok());
    }
}
