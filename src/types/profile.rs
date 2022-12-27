use serde::Deserialize;

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
    pub legacy: LegacyProfile
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
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub protected: bool,
    pub screen_name: String,
    pub statuses_count: i64,
    pub verified: bool,
}