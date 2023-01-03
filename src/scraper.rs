use std::cell::RefCell;

use super::types::auth::GuestToken;
use crate::{
    error::{ResponseError, Error},
    profile::Profile,
    types::{
        adaptive::AdaptiveParams, auth::CSRFAuth, graph::GraphResponse, params::Params,
        profile::TwitterUserResponse, timeline::TwitterTimelineResponse, tweet::Tweet,
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
    csrf_auth: Option<CSRFAuth>,
}

impl TwitterScraper {
    pub fn new() -> Self {
        let client = Self::get_reqwest_client();

        Self {
            client,
            guest_token: None.into(),
            csrf_auth: None,
        }
    }

    pub fn add_csrf_auth<T: Into<String>>(mut self, auth_token: T, csrf_token: T) -> Self {
        self.csrf_auth = Some(CSRFAuth {
            auth_token: auth_token.into(),
            csrf_token: csrf_token.into(),
        });
        self
    }

    pub async fn get_guest_token(&self) -> Result<()> {
        let guest_token = self
            .make_request(
                "https://api.twitter.com/1.1/guest/activate.json",
                Method::POST,
                &None
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
    async fn make_request<S, T>(&self, url: S, method: Method, csrf: &Option<CSRFAuth>) -> Result<T>
    where
        S: reqwest::IntoUrl,
        T: DeserializeOwned + 'static,
    {
        let req = self.client.request(method, url);

        let req = match &self.guest_token.take() {
            Some(token) => req.header(
                "X-Guest-Token",
                HeaderValue::from_str(token.guest_token.as_str()).unwrap(),
            ),
            None => req,
        };

        let req = match &csrf {
            Some(token) => req
                .header(
                    "cookie",
                    HeaderValue::from_str(
                        format!("auth_token={};ct0={}", token.auth_token, token.csrf_token)
                            .as_str(),
                    )
                    .unwrap(),
                )
                .header(
                    "x-csrf-token",
                    HeaderValue::from_str(token.csrf_token.as_str()).unwrap(),
                ),
            None => req,
        };
        let req = req.send().await?;
        let code = req.status();
        let response = req.text().await?;

        if code.as_u16() != 200 {
            let response_error: ResponseError =
                serde_json::from_str(&response).map_err(|_| Error::from(code))?;

            return Err(response_error.into());
        }

        match serde_json::from_str(&response) {
            Ok(t) => Ok(t),
            Err(error) => {
                println!("{}, {}", response, code);
                let response_error: ResponseError =
                    serde_json::from_str(&response).map_err(|_| error)?;

                Err(response_error.into())
            }
        }
    }
    async fn get_timeline_response<S>(&self, url: S) -> Result<TwitterTimelineResponse>
    where
        S: reqwest::IntoUrl,
    {
        self.make_request(url, Method::GET, &None).await
    }

    pub async fn get_users_tweets(
        &self,
        username: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<Tweet>, Option<String>)> {
        let user_id = self.get_profile(username).await?.user_id;

        let params = AdaptiveParams::user_tweets_params(&user_id, cursor);

        let url = format!(
            "https://api.twitter.com/2/timeline/profile/{}.json?{}",
            user_id,
            serde_url_params::to_string(&params)?
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
        let response: TwitterUserResponse = self.make_request(url, Method::GET, &None).await?;
        response.data.user.try_into()
    }

    pub async fn get_followers(
        &self,
        username: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        self.get_follower_following(username, false, cursor).await
    }

    pub async fn get_following(
        &self,
        username: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        self.get_follower_following(username, true, cursor).await
    }

    async fn get_follower_following(
        &self,
        username: &str,
        following: bool,
        cursor: Option<String>,
    ) -> Result<(Vec<Profile>, Option<String>)> {
        let user_id = self.get_profile(username).await?.user_id;
        let params = Params::new(user_id, cursor).to_url()?;
        let url = match following {
            true => "https://api.twitter.com/graphql/cocC_CzoxzpwgXr3jhG7DA/Following",
            false => "https://api.twitter.com/graphql/KwJEsSEIHz991Ansf4Y1tQ/Followers",
        };
        let url = format!("{}?{}", url, params);

        self.make_request::<_, GraphResponse>(url, Method::GET, &self.csrf_auth)
            .await
            .map(|r| r.get_users())?
    }
}

impl Default for TwitterScraper {
    fn default() -> Self {
        Self::new()
    }
}
