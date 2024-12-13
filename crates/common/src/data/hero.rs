#[derive(serde::Deserialize, Debug)]
pub struct GetHeroesResponse {
    result: GetHeroesResult,
}

#[derive(serde::Deserialize, Debug)]
struct GetHeroesResult {
    heroes: Vec<Hero>,
    status: i32,
    count: i32,
}

#[derive(serde::Deserialize, Debug)]
struct Hero {
    name: String,
    id: i32,
    localized_name: String,
}
