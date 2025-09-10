use lolclientapi_rs::blocking::LeagueClient;

#[test]
fn test_lcu_connection() {
    let client = LeagueClient::new();
    dbg!(&client.config.port);
    dbg!(&client.config.auth_token);
    dbg!(&client.config.auth_token_encoded);
    assert!(client.status());
}

mod lol_champ_select {
    use super::LeagueClient;

    #[test]
    fn test_get_lol_champ_select_v1_pickable_skin_ids() {
        let client = LeagueClient::new();
        let res = client.get_lol_champ_select_v1_pickable_skin_ids();
        dbg!(&res);
        assert!(res.is_ok())
    }

    #[test]
    fn test_get_lol_champ_select_v1_current_champion() {
        let client = LeagueClient::new();
        let res = client.get_lol_champ_select_v1_current_champion();
        dbg!(&res);
        assert!(res.is_ok())
    }

    #[test]
    fn test_get_lol_champ_select_v1_session() {
        let client = LeagueClient::new();
        let res = client.get_lol_champ_select_v1_session();
        dbg!(&res);
        assert!(res.is_ok())
    }
}

mod lol_champions {
    use super::LeagueClient;

    #[test]
    fn test_get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins() {
        let client = LeagueClient::new();
        let summoner_id = client.get_lol_summoner_v1_current_summoner_account_and_summoner_ids().unwrap().summoner_id;
        let res = client.get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins(summoner_id as i64, 1);
        dbg!(&res);
        assert!(res.is_ok())
    }
}

mod lol_summoner {
    use super::LeagueClient;

    #[test]
    fn test_get_lol_summoner_v1_current_summoner_account_and_summoner_ids() {
        let client = LeagueClient::new();
        let res = client.get_lol_summoner_v1_current_summoner_account_and_summoner_ids();
        dbg!(&res);
        assert!(res.is_ok())
    }
}

mod lol_chat {
    use super::LeagueClient;

    #[test]
    fn test_get_lol_chat_v1_friends() {
        let client = LeagueClient::new();
        let res = client.get_lol_chat_v1_friends();
        dbg!(&res);
        assert!(res.is_ok())
    }

    #[test]
    fn test_get_lol_chat_v1_me() {
        let client = LeagueClient::new();
        let res = client.get_lol_chat_v1_me();
        dbg!(&res);
        assert!(res.is_ok())
    }
}
