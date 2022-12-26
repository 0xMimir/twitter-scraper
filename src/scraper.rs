use std::cell::RefCell;

use super::types::guest_token::GuestToken;
use crate::{
    error::ResponseError,
    profile::Profile,
    types::{
        adaptive::AdaptiveParams, profile::TwitterUserResponse, timeline::TwitterTimelineResponse,
        tweet::Tweet,
    },
    Result,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, Method,
};
use serde::de::DeserializeOwned;

const BEARER_TOKEN: &str = "Bearer AAAAAAAAAAAAAAAAAAAAAPYXBAAAAAAACLXUNDekMxqa8h%2F40K4moUkGsoc%3DTYfbDKbT3jJPCEVnMYqilB28NHfOPqkca3qaAxGfsyKCs0wRbw";

pub struct TwitterScraper {
    client: Client,
    guest_token: RefCell<Option<GuestToken>>,
}

impl TwitterScraper {
    pub fn new() -> Self {
        let client = Self::get_reqwest_client();

        Self {
            client,
            guest_token: None.into(),
        }
    }

    pub async fn get_guest_token(&self) -> Result<()> {
        let guest_token = self
            .make_request(
                "https://api.twitter.com/1.1/guest/activate.json",
                Method::POST,
            )
            .await?;

        self.guest_token.replace(Some(guest_token));
        Ok(())
    }

    fn get_reqwest_client() -> Client {
        let mut headers = HeaderMap::new();
        headers.append("Authorization", HeaderValue::from_static(BEARER_TOKEN));

        ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap()
    }
    async fn make_request<S, T>(&self, url: S, method: Method) -> Result<T>
    where
        S: reqwest::IntoUrl,
        T: DeserializeOwned + 'static,
    {
        let req = self.client.request(method, url);

        let response = match &self.guest_token.take() {
            Some(token) => req.header(
                "X-Guest-Token",
                HeaderValue::from_str(token.guest_token.as_str()).unwrap(),
            ),
            None => req,
        }
        .send()
        .await?
        .text()
        .await?;

        match serde_json::from_str(&response) {
            Ok(t) => Ok(t),
            Err(_) => {
                let response_error: ResponseError = serde_json::from_str(&response)?;
                Err(response_error.into())
            }
        }
    }
    async fn get_timeline_response<S>(&self, url: S) -> Result<TwitterTimelineResponse>
    where
        S: reqwest::IntoUrl,
    {
        self.make_request(url, Method::GET).await
    }

    pub async fn get_users_tweets(
        &self,
        username: &str,
        _cursor: Option<String>,
    ) -> Result<(Vec<Tweet>, Option<String>)> {
        let user_id = self.get_profile(username).await?.user_id;
        let url = format!(
            "https://api.twitter.com/2/timeline/profile/{}.json",
            user_id
        );
        self.get_timeline_response(url)
            .await
            .map(|x| x.parse_tweets())
    }

    pub async fn search(
        &self,
        query: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<Tweet>, Option<String>)> {
        let params = AdaptiveParams::search_params(query, cursor);
        let url = format!(
            "https://twitter.com/i/api/2/search/adaptive.json?{}",
            serde_url_params::to_string(&params)?
        );
        self.get_timeline_response(url)
            .await
            .map(|x| x.parse_tweets())
    }

    pub async fn get_profile(&self, username: &str) -> Result<Profile> {
        let url = format!("https://api.twitter.com/graphql/4S2ihIKfF3xhp-ENxvUAfQ/UserByScreenName?variables=%7B%22screen_name%22%3A%22{}%22%2C%22withHighlightedLabel%22%3Atrue%7D", username);
        let response: TwitterUserResponse = self.make_request(url, Method::GET).await?;
        response.try_into()
    }
}

impl Default for TwitterScraper {
    fn default() -> Self {
        Self::new()
    }
}
