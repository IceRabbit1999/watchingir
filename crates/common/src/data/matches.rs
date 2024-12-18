use std::collections::VecDeque;

use serde::Deserialize;
use snafu::OptionExt;

use crate::error::NoneValueSnafu;

#[derive(Deserialize, Debug)]
pub struct MatchHistoryResponse {
    result: MatchHistoryResult,
}

impl MatchHistoryResponse {
    pub fn match_seq_num(&self) -> Vec<i64> {
        self.result.matches.iter().map(|m| m.match_seq_num).collect()
    }
}

#[derive(Deserialize, Debug)]
struct MatchHistoryResult {
    matches: Vec<Match>,
    status: i32,
    num_results: i32,
    total_results: i32,
    results_remaining: i32,
}

#[derive(Deserialize, Debug)]
struct Match {
    match_id: i64,
    match_seq_num: i64,
    start_time: i64,
    lobby_type: i32,
    radiant_team_id: i32,
    dire_team_id: i32,
    players: Vec<Player>,
}

#[derive(Deserialize, Debug)]
struct Player {
    account_id: i64,
    player_slot: i32,
    hero_id: i32,
    hero_variant: i32,
}

#[derive(Deserialize, Debug)]
pub struct MatchDetailResponse {
    result: MatchDetailResult,
}

impl MatchDetailResponse {
    pub fn views(
        self,
        account_id: i64,
    ) -> VecDeque<MatchDetailView> {
        let matches = self.result.matches;
        matches
            .into_iter()
            .map(|m| MatchDetailView::from_match_detail(m, account_id).expect("No account_id matched"))
            .collect::<VecDeque<MatchDetailView>>()
    }
}

#[derive(Deserialize, Debug)]
struct MatchDetailResult {
    status: i32,
    matches: Vec<MatchDetail>,
}

#[derive(Deserialize, Debug)]
struct MatchDetail {
    players: Vec<PlayerDetail>,
    radiant_win: bool,
    duration: i32,
    start_time: i64,
    match_id: i64,
    match_seq_num: i64,
    first_blood_time: i32,
    lobby_type: LobbyType,
    game_mode: GameMode,
    radiant_score: i32,
    dire_score: i32,
}

pub struct MatchDetailView {
    win: bool,
    duration: i32,
    start_time: i64,
    game_mode: GameMode,
    radiant_score: i32,
    dire_score: i32,
    player_detail: PlayerDetail,
}

impl MatchDetailView {
    fn from_match_detail(
        match_detail: MatchDetail,
        account_id: i64,
    ) -> Result<Self, crate::Error> {
        let player_detail = match_detail
            .players
            .into_iter()
            .find(|p| p.account_id == account_id)
            .context(NoneValueSnafu { expected: "PlayerDetail" })?;
        Ok(Self {
            win: match_detail.radiant_win && player_detail.player_slot < 128 || !match_detail.radiant_win && player_detail.player_slot >= 128,
            duration: match_detail.duration,
            start_time: match_detail.start_time,
            game_mode: match_detail.game_mode,
            radiant_score: match_detail.radiant_score,
            dire_score: match_detail.dire_score,
            player_detail,
        })
    }

    pub fn win_col(&self) -> String {
        if self.win {
            "Win".to_owned()
        } else {
            "Lose".to_owned()
        }
    }

    pub fn start_time_col(&self) -> String {
        let date_time = chrono::DateTime::from_timestamp(self.start_time, 0);
        date_time
            .map(|dt| chrono::DateTime::<chrono::Local>::from(dt).format("%Y/%m/%d %H:%M:%S").to_string())
            .unwrap_or(String::from("Unknown"))
    }

    pub fn duration_col(&self) -> String {
        format!("{}m{}s", self.duration / 60, self.duration % 60)
    }

    pub fn game_mode_col(&self) -> String {
        format!("{:?}", self.game_mode)
    }

    pub fn player_detail_col(&self) -> String {
        format!("{:#?}", self.player_detail)
    }

    pub fn player_detail(&self) -> PlayerDetail {
        self.player_detail.clone()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerDetail {
    pub account_id: i64,
    pub player_slot: i32,
    pub hero_id: i32,
    pub hero_variant: i32,
    pub item_0: i32,
    pub item_1: i32,
    pub item_2: i32,
    pub item_3: i32,
    pub item_4: i32,
    pub item_5: i32,
    pub backpack_0: i32,
    pub backpack_1: i32,
    pub backpack_2: i32,
    pub item_neutral: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub leaver_status: LeaverStatus,
    pub last_hits: i32,
    pub denies: i32,
    pub gold_per_min: i32,
    pub xp_per_min: i32,
    pub level: i32,
    pub net_worth: i32,
    pub aghanims_scepter: i32,
    pub aghanims_shard: i32,
    pub moonshard: i32,
    pub hero_damage: i32,
    pub tower_damage: i32,
    pub hero_healing: i32,
    pub gold: i32,
    pub gold_spent: i32,
}

#[derive(Debug)]
enum LobbyType {
    Invalid = -1,
    PublicMatchmaking = 0,
    Practice = 1,
    Tournament = 2,
    Tutorial = 3,
    CoopWithBots = 4,
    TeamMatch = 5,
    SoloQueue = 6,
    Ranked = 7,
    SoloMid1v1 = 8,
}

impl<'de> Deserialize<'de> for LobbyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            -1 => Ok(LobbyType::Invalid),
            0 => Ok(LobbyType::PublicMatchmaking),
            1 => Ok(LobbyType::Practice),
            2 => Ok(LobbyType::Tournament),
            3 => Ok(LobbyType::Tutorial),
            4 => Ok(LobbyType::CoopWithBots),
            5 => Ok(LobbyType::TeamMatch),
            6 => Ok(LobbyType::SoloQueue),
            7 => Ok(LobbyType::Ranked),
            8 => Ok(LobbyType::SoloMid1v1),
            _ => Err(serde::de::Error::custom("invalid value for LobbyType")),
        }
    }
}

#[derive(Debug)]
pub enum GameMode {
    None = 0,
    AllPick = 1,
    CaptainMode = 2,
    RandomDraft = 3,
    SingleDraft = 4,
    AllRandom = 5,
    Intro = 6,
    Diretide = 7,
    ReverseCaptainMode = 8,
    Greeviling = 9,
    Tutorial = 10,
    MidOnly = 11,
    LeastPlayed = 12,
    NewPlayerPool = 13,
    CompendiumMatchmaking = 14,
    CoopVsBots = 15,
    CaptainsDraft = 16,
    AbilityDraft = 18,
    AllRandomDeathMatch = 20,
    OneVsOneMid = 21,
    RankedMatchmaking = 22,
    Turbo = 23,
}

impl<'de> Deserialize<'de> for GameMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(GameMode::None),
            1 => Ok(GameMode::AllPick),
            2 => Ok(GameMode::CaptainMode),
            3 => Ok(GameMode::RandomDraft),
            4 => Ok(GameMode::SingleDraft),
            5 => Ok(GameMode::AllRandom),
            6 => Ok(GameMode::Intro),
            7 => Ok(GameMode::Diretide),
            8 => Ok(GameMode::ReverseCaptainMode),
            9 => Ok(GameMode::Greeviling),
            10 => Ok(GameMode::Tutorial),
            11 => Ok(GameMode::MidOnly),
            12 => Ok(GameMode::LeastPlayed),
            13 => Ok(GameMode::NewPlayerPool),
            14 => Ok(GameMode::CompendiumMatchmaking),
            15 => Ok(GameMode::CoopVsBots),
            16 => Ok(GameMode::CaptainsDraft),
            18 => Ok(GameMode::AbilityDraft),
            20 => Ok(GameMode::AllRandomDeathMatch),
            21 => Ok(GameMode::OneVsOneMid),
            22 => Ok(GameMode::RankedMatchmaking),
            23 => Ok(GameMode::Turbo),
            _ => Err(serde::de::Error::custom("invalid value for GameMode")),
        }
    }
}

#[derive(Debug, Clone)]
enum LeaverStatus {
    None = 0,
    Disconnected = 1,
    DisconnectedTooLong = 2,
    Abandoned = 3,
    Afk = 4,
    NeverConnected = 5,
    NeverConnectedTooLong = 6,
}

impl<'de> Deserialize<'de> for LeaverStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(LeaverStatus::None),
            1 => Ok(LeaverStatus::Disconnected),
            2 => Ok(LeaverStatus::DisconnectedTooLong),
            3 => Ok(LeaverStatus::Abandoned),
            4 => Ok(LeaverStatus::Afk),
            5 => Ok(LeaverStatus::NeverConnected),
            6 => Ok(LeaverStatus::NeverConnectedTooLong),
            _ => Err(serde::de::Error::custom("invalid value for LeaverStatus")),
        }
    }
}
