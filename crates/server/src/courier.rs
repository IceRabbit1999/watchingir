use common::data::{
    constant::{ConstantRequest, ConstantResponse},
    matches::{MatchDetailResponse, MatchHistoryResponse},
};
use snafu::{OptionExt, ResultExt};

use crate::error::{DataFormatSnafu, NoneValueSnafu, SteamApiSnafu};

const IDOTA2MATCH: &str = "https://api.steampowered.com/IDOTA2Match_570";
const STRATZ_API: &str = "https://api.stratz.com/graphql";

pub struct Courier {
    client: reqwest::Client,
}

impl Default for Courier {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Courier {
    // matches

    pub async fn latest_friends_match_detail(
        &self,
        key: &str,
        friend_ids: &[i64],
    ) -> Result<Vec<(MatchDetailResponse, i64)>, crate::Error> {
        let mut responses = Vec::new();
        for &account_id in friend_ids {
            let response = self.latest_match_detail(key, account_id).await?;
            responses.push((response, account_id));
        }
        Ok(responses)
    }

    pub async fn latest_match_detail(
        &self,
        key: &str,
        account_id: i64,
    ) -> Result<MatchDetailResponse, crate::Error> {
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
    ) -> Result<MatchHistoryResponse, crate::Error> {
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
    ) -> Result<MatchDetailResponse, crate::Error> {
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

    // cache

    pub async fn constant(
        &self,
        key: &str,
    ) -> Result<ConstantResponse, crate::Error> {
        let request = ConstantRequest::default();

        let response = self
            .client
            .post(STRATZ_API)
            .header("User-Agent", "STRATZ_API")
            .header("Authorization", format!("Bearer {}", key))
            .json(&request)
            .send()
            .await
            .context(SteamApiSnafu { entrypoint: "cache" })?;

        let response = response
            .json::<ConstantResponse>()
            .await
            .context(DataFormatSnafu { data: "ConstantResponse" })?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn latest_detail() {
        dotenvy::dotenv().ok();
        let account_id: i64 = std::env::var("ACCOUNT_ID").unwrap().parse().unwrap();
        let key = std::env::var("STEAM_KEY").unwrap();

        println!("account_id: {}", account_id);
        println!("key: {}", key);

        let client = reqwest::Client::new();
        let courier = super::Courier { client };
        let response = courier.latest_match_detail(&key, account_id).await.unwrap();
        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn cache() {
        dotenvy::dotenv().ok();
        let key = std::env::var("STRATZ_KEY").unwrap();

        let client = reqwest::Client::new();
        let courier = super::Courier { client };
        let response = courier.constant(&key).await.unwrap();
        println!("{:#?}", response);
    }
}
