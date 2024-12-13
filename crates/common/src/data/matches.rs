use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MatchHistoryResponse {
    result: MatchHistoryResult,
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
    leagueid: i32,
    game_mode: GameMode,
    radiant_score: i32,
    dire_score: i32,
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
    moon_shard: i32,
    hero_damage: i32,
    tower_damage: i32,
    hero_healing: i32,
    gold: i32,
    gold_spent: i32,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
enum GameMode {
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

#[derive(Deserialize, Debug)]
enum LeaverStatus {
    None = 0,
    Disconnected = 1,
    DisconnectedTooLong = 2,
    Abandoned = 3,
    Afk = 4,
    NeverConnected = 5,
    NeverConnectedTooLong = 6,
}
