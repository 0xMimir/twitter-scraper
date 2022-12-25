use chrono::NaiveDateTime;

use super::{video::Video, place::Place};

pub struct Tweet {
    pub hashtags: Vec<String>,
    pub html: String,
    pub id: String,
    pub in_reply_to_status: String,
    pub is_quoted: bool,
    pub is_pin: bool,
    pub is_reply: bool,
    pub is_retweet: bool,
    pub likes: i64,
    pub permanent_url: String,
    pub photos: Vec<String>,
    pub replies: i64,
    pub retweets: i64,
    pub text: String,
    pub time_parsed: NaiveDateTime,
    pub timestamp: i64,
    pub urls: Vec<String>,
    pub user_id: String,
    pub username: String,
    pub videos: Vec<Video>,
    pub sensitive_content: bool,
    pub place: Place
}