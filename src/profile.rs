use crate::BEARER_TOKEN;
use anyhow::{Context, Result};
use log::debug;
use serde_json::{json, Value};
use super::TwAPI;

const SEARCH_URL: &str = "https://twitter.com/i/api/graphql/k3027HdkVqbuDPpdoniLKA/Viewer";

impl TwAPI {
    pub async fn me(&self) -> Result<Value> {

        let variables = json!(
            {"withCommunitiesMemberships": true,
                     "withSubscribedTab": true, "withCommunitiesCreation": true}
        );

        let features = json!({"responsive_web_graphql_exclude_directive_enabled": true, "verified_phone_label_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "responsive_web_graphql_timeline_navigation_enabled": true, "user_data_features": true});

        let q = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
        ];
        let req = self
            .client
            .get(SEARCH_URL)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .query(&q)
            .build()
            ?;
        let text = self.client
            .execute(req).await?
            .text().await?;
        let res: Value = serde_json::from_str(&text).context("can't convert response to json")?;
        debug!("me res {res}");
        return Ok(res);
    }

    pub fn me_following(&mut self) {

    }

    pub async fn me_rest_id(&mut self) -> Result<i64> {
        let me = self.me().await?;
        let res_id = me
            .get("data").context("data")?
            .get("viewer").context("viewer")?
            .get("user_results").context("viewer")?
            .get("result").context("viewer")?
            .get("rest_id").context("rest id")?.as_str().context("str error")?.parse::<i64>()?;
        Ok(res_id)
        
    }
}
