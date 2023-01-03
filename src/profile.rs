use crate::{types::profile::TwitterUser, Error, Result};
use chrono::NaiveDateTime;

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

impl TryFrom<TwitterUser> for Profile {
    type Error = Error;

    fn try_from(value: TwitterUser) -> Result<Self> {
        let legacy = value.legacy;
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
            user_id: value.rest_id,
            username: legacy.screen_name,
            website: None,
        })
    }
}

#[tokio::test]
async fn test_profile_not_found() {
    use crate::TwitterScraper;
    let scraper = TwitterScraper::new();
    scraper.get_guest_token().await.unwrap();
    let profile = scraper.get_profile("ADLP40329lfdslfkdDJKSAHDkJ").await;
    let error = profile.unwrap_err();
    match error{
        Error::UserNotFound => (),
        _ => assert!(false)
    }
}
#[tokio::test]
async fn test_profile_error_suspended() {
    use crate::TwitterScraper;
    let scraper = TwitterScraper::new();
    scraper.get_guest_token().await.unwrap();
    let profile = scraper.get_profile("123").await;
    assert_eq!(profile.is_err(), true);
    let error = profile.unwrap_err();
    match error{
        Error::UserSuspended => (),
        _ => assert!(false)
    }
}
#[tokio::test]
async fn test_profile_valid() {
    use crate::TwitterScraper;
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
