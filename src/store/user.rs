use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::Error;

#[derive(Deserialize, Debug)]
pub struct TwitterUserResponse {
    pub data: TwitterUserData,
}

#[derive(Deserialize, Debug)]
pub struct TwitterUserData {
    pub user: TwitterUser,
}

#[derive(Deserialize, Debug)]
pub struct TwitterUser {
    pub id: String,
    pub rest_id: String,
    pub legacy: LegacyProfile,
    pub is_profile_translatable: bool,
}

#[derive(Deserialize, Debug)]
pub struct LegacyProfile {
    pub created_at: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: String,
    pub fast_followers_count: i64,
    pub favourites_count: i64,
    pub followers_count: i64,
    pub friends_count: i64,
    pub has_custom_timelines: bool,
    pub is_translator: bool,
    pub listed_count: i64,
    pub location: String,
    pub media_count: i64,
    pub name: String,
    pub normal_followers_count: i64,
    pub pinned_tweet_ids_str: Vec<String>,
    pub profile_banner_url: String,
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub protected: bool,
    pub screen_name: String,
    pub statuses_count: i64,
    pub verified: bool,
}

#[derive(Debug)]
pub struct User {
    pub avatar: String,
    pub banner: String,
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
    pub tweet_count: i64,
    pub url: String,
    pub user_id: String,
    pub username: String,
    pub website: Option<String>,
}

impl TryFrom<TwitterUser> for User {
    type Error = Error;

    fn try_from(value: TwitterUser) -> Result<Self, Self::Error> {
        let created_at =
            NaiveDateTime::parse_from_str(&value.legacy.created_at, "%a %b %d %T %z %Y")?;

        let user = value.legacy;
        Ok(Self {
            avatar: user.profile_image_url_https,
            banner: user.profile_banner_url,
            biography: user.description,
            followers_count: user.followers_count,
            following_count: user.favourites_count,
            is_private: user.protected,
            is_verified: user.verified,
            likes_count: user.followers_count,
            listed_count: user.listed_count,
            location: user.location,
            name: user.name,
            pinned_tweet_ids: user.pinned_tweet_ids_str,
            tweet_count: user.statuses_count,
            url: format!("https://twitter.com/{}", user.screen_name),
            user_id: value.id,
            username: user.screen_name,
            friends_count: user.friends_count,
            joined: created_at,
            website: None
        })
    }
}
