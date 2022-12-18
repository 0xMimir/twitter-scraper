use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitterTimelineResponse {
    pub global_objects: GlobalObjects,
    pub timeline: Timeline,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalObjects {
    pub tweets: HashMap<String, Tweet>,
    pub users: HashMap<String, User>,
}

#[derive(Debug, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub id: i64,
    pub id_str: String,
    pub text: String,
    pub truncated: bool,
    pub entities: Entities,
    pub source: String,
    pub in_reply_to_status_id: Option<i64>,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_user_id: Option<i64>,
    pub in_reply_to_user_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,
    pub user_id: i64,
    pub user_id_str: String,
    pub is_quote_status: bool,
    pub quoted_status_id: Option<i64>,
    pub quoted_status_id_str: Option<String>,
    pub retweet_count: i64,
    pub favorite_count: i64,
    pub conversation_id: i64,
    pub conversation_id_str: String,
    pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub possibly_sensitive_editable: Option<bool>,
    pub lang: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Entities {
    pub hashtags: Option<Vec<Value>>,
    pub symbols: Option<Vec<Value>>,
    pub user_mentions: Option<Vec<Value>>,
    pub urls: Option<Vec<Url>>,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    pub indices: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
    pub description: String,
    pub url: Option<String>,
    pub entities: Entities,
    pub protected: bool,
    pub followers_count: i64,
    pub friends_count: i64,
    pub listed_count: i64,
    pub created_at: String,
    pub favourites_count: i64,
    pub geo_enabled: bool,
    pub verified: bool,
    pub statuses_count: i64,
    pub media_count: i64,
    pub lang: Option<String>,
    pub contributors_enabled: bool,
    pub is_translator: bool,
    pub is_translation_enabled: bool,
    pub has_extended_profile: bool,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub has_custom_timelines: bool,
    pub business_profile_state: String,
    pub translator_type: String,
    pub require_some_consent: bool,
}

#[derive(Debug, Deserialize)]
pub struct Timeline {
    pub id: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub add_entries: Option<AddEntries>,
}

#[derive(Debug, Deserialize)]
pub struct AddEntries {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub entry_id: String,
    pub sort_index: String,
    pub content: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub operation: Option<Operation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub cursor: Cursor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cursor {
    pub value: String,
    pub cursor_type: String,
    pub stop_on_empty_response: Option<bool>,
}

impl TwitterTimelineResponse {
    pub fn parse_tweets(self, user_id: Option<String>) -> (Vec<Tweet>, Option<String>) {
        let mut cursor = String::new();
        let tweets = self.global_objects.tweets.into_values();

        let tweets = match user_id {
            Some(user_id) => tweets.filter(|t| t.user_id_str == user_id).collect(),
            None => tweets.collect(),
        };

        for instruction in self.timeline.instructions {
            if let Some(entries) = instruction.add_entries {
                for entry in entries.entries {
                    if let Some(operation) = entry.content.operation {
                        if operation
                            .cursor
                            .cursor_type
                            .to_lowercase()
                            .starts_with("bottom")
                        {
                            cursor = operation.cursor.value;
                        }
                    }
                }
            }
        }

        let cursor = match cursor.is_empty() {
            true => None,
            false => Some(cursor),
        };

        (tweets, cursor)
    }
}
