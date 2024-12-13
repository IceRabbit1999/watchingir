use common::data::matches::{MatchDetailResponse, MatchHistoryResponse};

const IDOTA2MATCH: &str = "https://api.steampowered.com/IDOAT2Match_570";

pub struct Courier {
    client: reqwest::Client,
}

impl Courier {
    pub fn from_setting() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    // matches
    pub async fn latest_match_detail(
        &self,
        key: &str,
        account_id: u64,
    ) -> anyhow::Result<MatchDetailResponse> {
        let match_history_response = self.get_match_history(key, account_id, 1).await?;

        todo!()
    }

    async fn get_match_history(
        &self,
        key: &str,
        account_id: u64,
        matches_requested: i32,
    ) -> anyhow::Result<MatchHistoryResponse> {
        let url = format!(
            "{}/GetMatchHistory/V001/?key={}&account_id={}&matches_requested={}",
            IDOTA2MATCH, key, account_id, matches_requested
        );

        todo!()
    }

    async fn get_match_detail(
        &self,
        key: &str,
        sequence: u64,
    ) -> anyhow::Result<MatchDetailResponse> {
        // GetMatchDetails is broken
        let url = format!(
            "{}/GetMatchHistoryBySequenceNum/V001/?key={}&start_at_match_seq_num={}",
            IDOTA2MATCH, key, sequence
        );

        todo!()
    }
}

#[cfg(test)]
mod tests {
    fn key() -> String {
        std::fs::read_to_string("../../config/dev.toml")
            .unwrap()
            .lines()
            .find(|line| line.starts_with("steam_api_key"))
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .trim()
            .replace("\"", "")
            .to_string()
    }
}
