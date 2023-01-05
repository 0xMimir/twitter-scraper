use super::tweet::Tweet;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitterTimelineResponse {
    global_objects: GlobalObjects,
    timeline: Timeline,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GlobalObjects {
    tweets: HashMap<String, TweetRaw>,
    users: HashMap<String, User>,
}

#[derive(Debug, Deserialize)]
struct User {
    // pub id: i64,
    // pub id_str: String,
    // pub name: String,
    pub screen_name: String,
    // pub description: String,
    // pub url: Option<String>,
    // pub entities: Entities,
    // pub protected: bool,
    // pub followers_count: i64,
    // pub friends_count: i64,
    // pub listed_count: i64,
    // pub created_at: String,
    // pub favourites_count: i64,
    // pub verified: bool,
    // pub media_count: i64,
    // pub lang: Option<String>,
    // pub contributors_enabled: bool,
    // pub is_translator: bool,
    // pub is_translation_enabled: bool,
    // pub has_extended_profile: bool,
    // pub default_profile: bool,
    // pub default_profile_image: bool,
    // pub has_custom_timelines: bool,
    // pub business_profile_state: String,
    // pub translator_type: String,
    // pub require_some_consent: bool,
}

#[derive(Debug, Deserialize)]
struct TweetRaw {
    pub created_at: String,
    pub id: i64,
    pub id_str: String,
    pub text: String,
    // pub truncated: bool,
    pub entities: Entities,
    pub source: String,
    // pub in_reply_to_status_id: Option<i64>,
    pub in_reply_to_status_id_str: Option<String>,
    // pub in_reply_to_user_id: Option<i64>,
    // pub in_reply_to_user_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,
    pub user_id: i64,
    pub user_id_str: String,
    pub is_quote_status: bool,
    // pub quoted_status_id: Option<i64>,
    // pub quoted_status_id_str: Option<String>,
    pub retweet_count: i64,
    pub favorite_count: i64,
    // pub conversation_id: i64,
    // pub conversation_id_str: String,
    // pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    // pub possibly_sensitive_editable: Option<bool>,
    // pub lang: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Entities {
    #[serde(default)]
    pub hashtags: Vec<Text>,
    #[serde(default)]
    pub symbols: Vec<Text>,
    #[serde(default)]
    pub user_mentions: Vec<UserMention>,
    #[serde(default)]
    pub urls: Vec<Url>,
}

#[derive(Debug, Deserialize)]
struct Url {
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct UserMention {
    pub screen_name: String,
}

#[derive(Debug, Deserialize)]
struct Text {
    pub text: String,
}

#[derive(Debug, Deserialize)]
struct Timeline {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Instruction {
    pub add_entries: Option<AddEntries>,
}

#[derive(Debug, Deserialize)]
struct AddEntries {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    pub content: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Content {
    pub operation: Option<Operation>,
    pub item: Option<TweetCursorItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Operation {
    pub cursor: Cursor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Cursor {
    pub value: String,
    pub cursor_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TweetCursorItem {
    pub content: TweetCursorContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TweetCursorContent {
    pub tweet: CursorTweet,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CursorTweet {
    pub id: String,
}

impl TwitterTimelineResponse {
    pub fn parse_tweets(self) -> (Vec<Tweet>, Option<String>) {
        let mut cursor = None;
        let mut tweets = vec![];
        for instruction in self.timeline.instructions.iter() {
            if let Some(entries) = &instruction.add_entries {
                for entry in entries.entries.iter() {
                    if let Some(tweet_entry) = &entry.content.item {
                        if let Some(tweet) = self.parse_tweet(&tweet_entry.content.tweet.id) {
                            tweets.push(tweet)
                        }
                    }
                    if let Some(operation) = &entry.content.operation {
                        if operation
                            .cursor
                            .cursor_type
                            .to_lowercase()
                            .starts_with("bottom")
                        {
                            cursor = Some(operation.cursor.value.to_owned());
                        }
                    }
                }
            }
        }

        (tweets, cursor)
    }
    pub fn parse_tweet(&self, tweet_id: &str) -> Option<Tweet> {
        let raw_tweet_info = self.global_objects.tweets.get(tweet_id)?;
        let username = self
            .global_objects
            .users
            .get(raw_tweet_info.user_id_str.as_str())?
            .screen_name
            .as_str();

        let time_parsed =
            match NaiveDateTime::parse_from_str(&raw_tweet_info.created_at, "%a %b %d %T %z %Y") {
                Ok(t) => Some(t),
                Err(_) => None,
            }?;

        let urls = raw_tweet_info
            .entities
            .urls
            .iter()
            .map(|h| h.url.to_owned())
            .collect();

        let hashtags = raw_tweet_info
            .entities
            .hashtags
            .iter()
            .map(|h| h.text.to_owned())
            .collect();

        let symbols = raw_tweet_info
            .entities
            .symbols
            .iter()
            .map(|s| s.text.to_owned())
            .collect();

        let mentions = raw_tweet_info
            .entities
            .user_mentions
            .iter()
            .map(|m| m.screen_name.to_owned())
            .collect();

        Some(Tweet {
            id: raw_tweet_info.id,
            in_reply_to_status: raw_tweet_info.in_reply_to_status_id_str.clone(),
            is_quoted: raw_tweet_info.is_quote_status,
            is_reply: raw_tweet_info.in_reply_to_screen_name.is_some(),
            is_retweet: raw_tweet_info.retweeted,
            likes: raw_tweet_info.favorite_count,
            permanent_url: format!(
                "https://twitter.com/{}/status/{}",
                username, raw_tweet_info.id_str
            ),
            retweets: raw_tweet_info.retweet_count,
            text: raw_tweet_info.text.to_owned(),
            timestamp: time_parsed.timestamp(),
            user_id: raw_tweet_info.user_id,
            username: username.to_owned(),
            sensitive_content: raw_tweet_info.possibly_sensitive.unwrap_or(false),
            source: raw_tweet_info.source.to_owned(),
            mentions,
            time_parsed,
            hashtags,
            symbols,
            urls,
        })
    }
}
