use common::{
    data::matches::{MatchDetailResponse, MatchHistoryResponse},
    error::{DataFormatSnafu, NoneValueSnafu, SteamApiSnafu},
};
use snafu::{OptionExt, ResultExt};
use tracing::info;

const IDOTA2MATCH: &str = "https://api.steampowered.com/IDOTA2Match_570";

pub struct Courier {
    client: reqwest::Client,
}

impl Courier {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    // matches
    pub async fn latest_match_detail(
        &self,
        key: &str,
        account_id: i64,
    ) -> Result<MatchDetailResponse, common::Error> {
        let match_history_response = self.get_match_history(key, account_id, 1).await?;
        let seq_num = match_history_response.match_seq_num();
        let seq_num = seq_num.first().context(NoneValueSnafu { expected: "match_seq_num" })?;

        let match_detail_response = self.get_match_detail(key, *seq_num, 1).await?;
        Ok(match_detail_response)
    }

    async fn get_match_history(
        &self,
        key: &str,
        account_id: i64,
        matches_requested: i32,
    ) -> Result<MatchHistoryResponse, common::Error> {
        let url = format!(
            "{}/GetMatchHistory/v1?key={}&account_id={}&matches_requested={}",
            IDOTA2MATCH, key, account_id, matches_requested
        );
        let response = self.client.get(&url).send().await.context(SteamApiSnafu {
            entrypoint: "GetMatchHistory",
        })?;
        let response = response.json::<MatchHistoryResponse>().await.context(DataFormatSnafu {
            data: "MatchHistoryResponse",
        })?;
        Ok(response)
    }

    /// `GetMatchDetails` is broken, more detail at: https://github.com/ValveSoftware/Dota-2/issues/2715
    /// Use `GetMatchHistoryBySequenceNum` instead
    async fn get_match_detail(
        &self,
        key: &str,
        sequence: i64,
        matches_requested: i32,
    ) -> Result<MatchDetailResponse, common::Error> {
        // GetMatchDetails is broken
        let url = format!(
            "{}/GetMatchHistoryBySequenceNum/v1?key={}&start_at_match_seq_num={}&matches_requested={}",
            IDOTA2MATCH, key, sequence, matches_requested
        );

        let response = self.client.get(&url).send().await.context(SteamApiSnafu {
            entrypoint: "GetMatchHistoryBySequenceNum",
        })?;
        let response = response
            .json::<MatchDetailResponse>()
            .await
            .context(DataFormatSnafu { data: "MatchDetailResponse" })?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn latest_detail() {
        dotenvy::dotenv().ok();
        let account_id: i64 = std::env::var("ACCOUNT_ID").unwrap().parse().unwrap();
        let key = std::env::var("API_KEY").unwrap();

        println!("account_id: {}", account_id);
        println!("key: {}", key);

        let client = reqwest::Client::new();
        let courier = super::Courier { client };
        let response = courier.latest_match_detail(&key, account_id).await.unwrap();
        println!("{:#?}", response);
    }
}
