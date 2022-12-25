use std::cell::RefCell;

use super::types::guest_token::GuestToken;
use crate::{error::ResponseError, Result};
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
    pub async fn get_guest_token(&self) -> Result<()> {
        let guest_token = self
            .make_request(
                "https://api.twitter.com/1.1/guest/activate.json",
                Method::POST,
            )
            .await?;

        self.guest_token.replace(Some(guest_token).into());
        Ok(())
    }
    pub fn new() -> Self {
        let client = Self::get_reqwest_client();

        Self {
            client,
            guest_token: None.into(),
        }
    }
    pub fn get_reqwest_client() -> Client {
        let mut headers = HeaderMap::new();
        headers.append("Authorization", HeaderValue::from_static(BEARER_TOKEN));

        ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap()
    }
    pub async fn make_request<S, T>(&self, url: S, method: Method) -> Result<T>
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

        println!("{}", response);

        match serde_json::from_str(&response) {
            Ok(t) => Ok(t),
            Err(_) => {
                let response_error: ResponseError = serde_json::from_str(&response)?;
                Err(response_error.into())
            }
        }
    }
    // async fn get_timeline_response<S>(&self, url: S) -> Result<TwitterTimelineResponse>
    // where
    //     S: reqwest::IntoUrl,
    // {
    //     let response = self.make_request(url, Method::GET).await?;
    //     serde_json::from_str(response.as_str()).map_err(Error::from)
    // }

    // pub async fn get_users_tweets(
    //     &self,
    //     username: &str,
    //     cursor: Option<String>,
    // ) -> Result<(Vec<Tweet>, Option<String>)> {
    //     let user_id = self.get_user(username).await?.user_id;
    //     let mut url = format!(
    //         "https://api.twitter.com/2/timeline/profile/{}.json?count=100",
    //         user_id
    //     );

    //     if let Some(cursor) = cursor {
    //         url = format!("{}&cursor={}", url, urlencoding::encode(cursor.as_str()))
    //     }

    //     let response = self.get_timeline_response(url).await?;
    //     Ok(response.parse_tweets(Some(user_id)))
    // }
    // pub async fn search_tweets(
    //     &self,
    //     query: &str,
    //     cursor: Option<String>,
    // ) -> Result<(Vec<Tweet>, Option<String>)> {
    //     let mut url = format!("https://twitter.com/i/api/2/search/adaptive.json?q={}&count=100&query_source=typed_query&pc=1&spelling_corrections=1", query);
    //     if let Some(cursor) = cursor {
    //         url = format!("{}&cursor={}", url, urlencoding::encode(cursor.as_str()));
    //     }

    //     let response = self.get_timeline_response(url).await?;
    //     Ok(response.parse_tweets(None))
    // }
}

impl Default for TwitterScraper {
    fn default() -> Self {
        Self::new()
    }
}
