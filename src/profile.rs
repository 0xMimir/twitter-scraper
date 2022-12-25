use crate::{types::profile::TwitterUserResponse, Error, Result, TwitterScraper};
use chrono::NaiveDateTime;
use reqwest::Method;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub avatar: String,
    pub banner: Option<String>,
    pub biography: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub friends_count: i64,
    pub is_private: bool,
    pub is_verified: bool,
    pub joined: NaiveDateTime,
    pub likes_count: i64,
    pub listed_count: i64,
    pub location: String,
    pub name: String,
    pub pinned_tweet_ids: Vec<String>,
    pub tweets_count: i64,
    pub url: String,
    pub user_id: String,
    pub username: String,
    pub website: Option<String>,
}

impl TryFrom<TwitterUserResponse> for Profile {
    type Error = Error;

    fn try_from(value: TwitterUserResponse) -> Result<Self> {
        let legacy = value.data.user.legacy;
        let joined = NaiveDateTime::parse_from_str(&legacy.created_at, "%a %b %d %T %z %Y")?;

        Ok(Self {
            avatar: legacy.profile_image_url_https,
            banner: legacy.profile_banner_url,
            biography: legacy.description,
            followers_count: legacy.followers_count,
            following_count: legacy.favourites_count,
            friends_count: legacy.friends_count,
            is_private: legacy.protected,
            is_verified: legacy.verified,
            joined,
            likes_count: legacy.favourites_count,
            listed_count: legacy.listed_count,
            location: legacy.location,
            name: legacy.name,
            pinned_tweet_ids: legacy.pinned_tweet_ids_str,
            tweets_count: legacy.statuses_count,
            url: format!("https://twitter.com/{}", legacy.screen_name),
            user_id: value.data.user.rest_id,
            username: legacy.screen_name,
            website: None,
        })
    }
}

impl TwitterScraper {
    pub async fn get_profile(&self, username: &str) -> Result<Profile> {
        let url = format!("https://api.twitter.com/graphql/4S2ihIKfF3xhp-ENxvUAfQ/UserByScreenName?variables=%7B%22screen_name%22%3A%22{}%22%2C%22withHighlightedLabel%22%3Atrue%7D", username);
        let response: TwitterUserResponse = self.make_request(url, Method::GET).await?;
        response.try_into()
    }
}

#[tokio::test]
async fn test_profile_not_found() {
    let scraper = TwitterScraper::new();
    scraper.get_guest_token().await.unwrap();
    let profile = scraper.get_profile("ADLP40329lfdslfkdDJKSAHDkJ").await;
    assert_eq!(profile, Err(Error::UserNotFound));
}
#[tokio::test]
async fn test_profile_error_suspended() {
    let scraper = TwitterScraper::new();
    scraper.get_guest_token().await.unwrap();
    let profile = scraper.get_profile("123").await;
    assert_eq!(profile, Err(Error::UserSuspended));
}
#[tokio::test]
async fn test_profile_valid() {
    let scraper = TwitterScraper::new();
    scraper.get_guest_token().await.unwrap();
    let profile = scraper.get_profile("elonmusk").await;
    assert!(profile.is_ok());
    let profile = profile.unwrap();
    assert_eq!(profile.is_private, false);
    assert!(profile.followers_count > 0);
    assert!(profile.friends_count > 0);
    assert!(profile.following_count > 0);
}
