use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Tweet {
    pub hashtags: Vec<String>,
    pub id: String,
    pub in_reply_to_status: Option<String>,
    pub is_quoted: bool,
    pub is_reply: bool,
    pub is_retweet: bool,
    pub likes: i64,
    pub permanent_url: String,
    pub retweets: i64,
    pub text: String,
    pub time_parsed: NaiveDateTime,
    pub symbols: Vec<String>,
    pub source: String,
    pub timestamp: i64,
    pub urls: Vec<String>,
    pub user_id: String,
    pub username: String,
    pub sensitive_content: bool,
}
