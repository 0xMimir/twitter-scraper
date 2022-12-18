use crate::{
    store::{
        guest_token::GuestToken,
        timeline::{Tweet, TwitterTimelineResponse},
        user::{TwitterUserResponse, User},
    },
    Error, Result,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, Method,
};

const BEARER_TOKEN: &str = "Bearer AAAAAAAAAAAAAAAAAAAAAPYXBAAAAAAACLXUNDekMxqa8h%2F40K4moUkGsoc%3DTYfbDKbT3jJPCEVnMYqilB28NHfOPqkca3qaAxGfsyKCs0wRbw";

pub struct TwitterScraper {
    client: Client,
    guest_token: Option<GuestToken>,
}

impl TwitterScraper {
    pub async fn get_guest_token(&mut self) -> Result<()> {
        let response = self
            .make_request(
                "https://api.twitter.com/1.1/guest/activate.json",
                Method::POST,
            )
            .await?;

        let guest_token: GuestToken = serde_json::from_str(response.as_str())?;
        self.client = Self::get_reqwest_client(Some(&guest_token.guest_token));
        self.guest_token = Some(guest_token);
        Ok(())
    }
    pub fn new() -> Self {
        let client = Self::get_reqwest_client(None);

        Self {
            client,
            guest_token: None,
        }
    }
    fn get_reqwest_client(guest_token: Option<&str>) -> Client {
        let mut headers = HeaderMap::new();
        headers.append("Authorization", HeaderValue::from_static(BEARER_TOKEN));

        if let Some(token) = guest_token {
            headers.append("X-Guest-Token", HeaderValue::from_str(token).unwrap());
        }

        ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap()
    }
    async fn make_request<S>(&self, url: S, method: Method) -> Result<String>
    where
        S: reqwest::IntoUrl,
    {
        self.client
            .request(method, url)
            .send()
            .await?
            .text()
            .await
            .map_err(Error::from)
    }
    async fn get_timeline_response<S>(&self, url: S) -> Result<TwitterTimelineResponse>
    where
        S: reqwest::IntoUrl,
    {
        let response = self.make_request(url, Method::GET).await?;
        serde_json::from_str(response.as_str()).map_err(Error::from)
    }
    pub async fn get_user(&self, username: &str) -> Result<User> {
        let url = format!("https://api.twitter.com/graphql/4S2ihIKfF3xhp-ENxvUAfQ/UserByScreenName?variables=%7B%22screen_name%22%3A%22{}%22%2C%22withHighlightedLabel%22%3Atrue%7D", username);
        let response = self.make_request(url, Method::GET).await?;
        let response: TwitterUserResponse = serde_json::from_str(response.as_str())?;
        User::try_from(response.data.user)
    }
    pub async fn get_users_tweets(
        &self,
        username: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<Tweet>, Option<String>)> {
        let user_id = self.get_user(username).await?.user_id;
        let mut url = format!(
            "https://api.twitter.com/2/timeline/profile/{}.json?count=100",
            user_id
        );

        if let Some(cursor) = cursor {
            url = format!("{}&cursor={}", url, urlencoding::encode(cursor.as_str()))
        }

        let response = self.get_timeline_response(url).await?;
        Ok(response.parse_tweets(Some(user_id)))
    }
    pub async fn search_tweets(
        &self,
        query: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<Tweet>, Option<String>)> {
        let mut url = format!("https://twitter.com/i/api/2/search/adaptive.json?q={}&count=100&query_source=typed_query&pc=1&spelling_corrections=1", query);
        if let Some(cursor) = cursor {
            url = format!("{}&cursor={}", url, urlencoding::encode(cursor.as_str()));
        }
        
        let response = self.get_timeline_response(url).await?;
        Ok(response.parse_tweets(None))
    }
}

impl Default for TwitterScraper {
    fn default() -> Self {
        Self::new()
    }
}
