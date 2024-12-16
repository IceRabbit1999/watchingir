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

impl MatchDetailResponse {}

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
    pub win: bool,
    pub duration: i32,
    pub start_time: i64,
    pub game_mode: GameMode,
    pub radiant_score: i32,
    pub dire_score: i32,
    player_detail: PlayerDetail,
}

impl MatchDetailView {
    pub fn from_match_detail(
        match_detail: MatchDetail,
        account_id: i64,
    ) -> Result<Self, crate::Error> {
        let player_detail = match_detail
            .players
            .into_iter()
            .find(|p| p.account_id == account_id)
            .context(NoneValueSnafu { expected: "PlayerDetail" })?;
        Ok(Self {
            win: match_detail.radiant_win,
            duration: match_detail.duration,
            start_time: match_detail.start_time,
            game_mode: match_detail.game_mode,
            radiant_score: match_detail.radiant_score,
            dire_score: match_detail.dire_score,
            player_detail,
        })
    }
}

#[derive(Deserialize, Debug)]
struct PlayerDetail {
    account_id: i64,
    hero_id: i32,
    hero_variant: i32,
    item_0: i32,
    item_1: i32,
    item_2: i32,
    item_3: i32,
    item_4: i32,
    item_5: i32,
    backpack_0: i32,
    backpack_1: i32,
    backpack_2: i32,
    item_neutral: i32,
    kills: i32,
    deaths: i32,
    assists: i32,
    leaver_status: LeaverStatus,
    last_hits: i32,
    denies: i32,
    gold_per_min: i32,
    xp_per_min: i32,
    level: i32,
    net_worth: i32,
    aghanims_scepter: i32,
    aghanims_shard: i32,
    moonshard: i32,
    hero_damage: i32,
    tower_damage: i32,
    hero_healing: i32,
    gold: i32,
    gold_spent: i32,
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

#[derive(Debug)]
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
