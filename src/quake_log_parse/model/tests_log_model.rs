#[cfg(test)]
mod tests {
    const LOG_DATA: &str = r#"6:34 InitGame: \capturelimit\8\g_maxGameClients\0\timelimit\15\fraglimit\20\dmflags\0\bot_minplayers\0\sv_allowDownload\0\sv_maxclients\16\sv_privateClients\2\g_gametype\= 0\sv_hostname\Code Miner Server\sv_minRate\0\sv_maxRate\10000\sv_minPing\0\sv_maxPing\0\sv_floodProtect\1\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0
6:34 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
6:34 ClientUserinfoChanged: 3 n\Oootsimo\t\0\model\razor/id\hmodel\razor/id\g_redteam\\g_blueteam\\c1\3\c2\5\hc\100\w\0\l\0\tt\0\tl\0
6:34 ClientUserinfoChanged: 4 n\Dono da Bola\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\95\w\0\l\0\tt\0\tl\0
6:34 ClientUserinfoChanged: 5 n\Assasinu Credi\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0
6:34 ClientUserinfoChanged: 6 n\Zeh\t\0\model\sarge/default\hmodel\sarge/default\g_redteam\\g_blueteam\\c1\1\c2\5\hc\100\w\0\l\0\tt\0\tl\0
6:34 ClientUserinfoChanged: 7 n\Mal\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0
6:43 Kill: 4 2 6: Dono da Bola killed Isgalamido by MOD_ROCKET
6:45 Kill: 1022 4 22: <world> killed Dono da Bola by MOD_TRIGGER_HURT
6:48 Kill: 6 3 6: Zeh killed Oootsimo by MOD_ROCKET
7:15 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT
7:16 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT
7:17 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT
7:22 Kill: 1022 3 22: <world> killed Oootsimo by MOD_TRIGGER_HURT
7:25 Kill: 1022 5 22: <world> killed Assasinu Credi by MOD_TRIGGER_HURT
7:26 Kill: 4 7 7: Dono da Bola killed Mal by MOD_ROCKET_SPLASH
7:29 Kill: 2 6 6: Isgalamido killed Zeh by MOD_ROCKET
7:33 Kill: 3 5 7: Oootsimo killed Assasinu Credi by MOD_ROCKET_SPLASH
7:34 Kill: 2 3 7: Isgalamido killed Oootsimo by MOD_ROCKET_SPLASH
7:37 Kill: 4 4 7: Dono da Bola killed Dono da Bola by MOD_ROCKET_SPLASH
7:40 Kill: 3 5 7: Oootsimo killed Assasinu Credi by MOD_ROCKET_SPLASH
7:44 Kill: 5 7 7: Assasinu Credi killed Mal by MOD_ROCKET_SPLASH
7:51 Kill: 3 4 7: Oootsimo killed Oootsimo by MOD_ROCKET_SPLASH
7:52 Kill: 1022 7 22: <world> killed Mal by MOD_TRIGGER_HURT
7:54 Kill: 5 3 7: Assasinu Credi killed Oootsimo by MOD_ROCKET_SPLASH
7:57 Kill: 7 6 7: Mal killed Zeh by MOD_ROCKET_SPLASH
13:52 Kill: 6 7 6: Zeh killed Mal by MOD_ROCKET
13:55 Kill: 3 4 6: Oootsimo killed Dono da Bola by MOD_ROCKET"#;
    use crate::quake_log_parse::model::{
        error::LogError,
        log_model::{
            insert_kills_by_means, process_events_matches, process_init_game, process_kill_line,
            process_ranking, Match, MatchData,
        },
    };
    #[test]
    fn test_process_events_matches() {
        let mut matches = Vec::new();
        let result = process_events_matches(&mut matches, LOG_DATA);

        assert!(result.is_ok());
        assert_eq!(matches.len(), 1);

        //<world> killed 7
        //7 kills deducted from players who were victims of <world>: <world> + 7.
        //Players killed 14
        //Total: 21.
        //The players "Oootsimo" and "Dono da Bola" committed suicide with a MOD_ROCKET_SPLASH
        //Killing oneself counts as a kill
        //Prove me wrong.

        let game_data = &matches[0].data;
        assert_eq!(game_data.kills["Dono da Bola"], 2);
        assert_eq!(game_data.kills["Oootsimo"], 3);
        assert_eq!(game_data.kills["Zeh"], 2);
        assert_eq!(game_data.kills["Mal"], 0);
        assert_eq!(game_data.kills["Assasinu Credi"], 1);
        assert_eq!(game_data.kills["Isgalamido"], -1);
        assert_eq!(game_data.total_kills, 21);
        assert_eq!(game_data.players.len(), 6);
    }
    #[test]
    fn test_process_init_game() {
        let mut matches = vec![];
        let result = process_init_game(&mut matches);
        assert!(result.is_ok());
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_process_kill_line_player_kill() {
        let mut game = MatchData::default();
        let line = "21:42 Kill: 1022 2 22: <world> killed NonExistentPlayer by MOD_TRIGGER_HURT";
        let result = process_kill_line(line, &mut game);
        assert!(result.is_err());

        game.players.insert("Isgalamido".to_owned());
        let line = "21:42 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT";
        let result = process_kill_line(line, &mut game);
        assert!(result.is_ok());
        assert_eq!(game.total_kills, 1);
        assert_eq!(game.kills.get("Isgalamido"), Some(&-1));
        assert_ne!(game.kills.get("<world>"), Some(&1));

        let line = "22:06 Kill: 2 3 7: Isgalamido killed Mocinha by MOD_ROCKET_SPLASH";
        let result = process_kill_line(line, &mut game);
        assert!(result.is_ok());
        assert_eq!(game.total_kills, 2);
        assert_eq!(game.kills.get("Isgalamido"), Some(&0));
        assert_ne!(game.kills.get("Mocinha"), Some(&0));

        let line = " 21:15 ClientUserinfoChanged: 2 n\\Isgalamido\\t\\0";
        let result = process_kill_line(line, &mut game);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_kills_by_means() {
        let mut game = MatchData::default();
        let line = "10:20 Kill: 1 2 3: Player1 killed Player2";
        let result = insert_kills_by_means(line, &mut game);
        assert_eq!(
            result.err(),
            Some(LogError::InsertKillMeanError(
                "Mean 'Player2' not recognized as a valid means of death: 10:20 Kill: 1 2 3: Player1 killed Player2"
                    .to_string()
            ))
        );
        assert!(game.kills_by_means.is_empty());
        let line = "10:20 Kill: 1 2 3: Player1 killed Player2 by MOD_ROCKET_SPLASH";
        let result = insert_kills_by_means(line, &mut game);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_ranking() {
        let mut matches = Vec::new();
        let mut ranking = Vec::new();

        let mut match_data = MatchData::default();
        match_data.players.insert("Player1".to_string());
        match_data.kills.insert("Player1".to_string(), 5);
        let match1 = Match {
            id: 1,
            data: match_data,
        };
        matches.push(match1);

        let mut match_data = MatchData::default();
        match_data.players.insert("Player2".to_string());
        match_data.kills.insert("Player2".to_string(), -3);
        let match2 = Match {
            id: 2,
            data: match_data,
        };
        matches.push(match2);

        let mut match_data = MatchData::default();
        match_data.players.insert("Player1".to_string());
        match_data.players.insert("Player2".to_string());
        match_data.kills.insert("Player1".to_string(), 2);
        match_data.kills.insert("Player2".to_string(), -1);
        match_data.players.insert("Player3".to_string());
        match_data.kills.insert("Player3".to_string(), 9);
        let match3 = Match {
            id: 3,
            data: match_data,
        };
        matches.push(match3);

        process_ranking(&mut matches, &mut ranking);

        assert_eq!(ranking.len(), 3);
        assert_eq!(ranking[0].name, "Player3");
        assert_eq!(ranking[0].kills, 9);
        assert_eq!(ranking[1].name, "Player1");
        assert_eq!(ranking[1].kills, 7);
        assert_eq!(ranking[2].name, "Player2");
        assert_eq!(ranking[2].kills, -4);
    }
}
