use serde::Serialize;

#[derive(Serialize)]
pub struct AdaptiveParams {
    pub q: Option<String>,
    pub count: Option<i32>,
    pub query_source: Option<String>,
    pub pc: Option<i32>,
    pub spelling_corrections: Option<i32>,
    pub cursor: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

impl AdaptiveParams {
    pub fn search_params<T: Into<String>>(q: T, cursor: Option<String>) -> Self {
        Self {
            q: Some(q.into()),
            count: Some(100),
            query_source: Some("typed_query".to_owned()),
            pc: Some(1),
            spelling_corrections: Some(1),
            cursor,
            user_id: None,
        }
    }
    pub fn add_cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
    pub fn new() -> Self {
        Self {
            q: None,
            count: None,
            query_source: None,
            pc: None,
            spelling_corrections: None,
            cursor: None,
            user_id: None,
        }
    }
    pub fn user_tweets_params<T: Into<String>>(user_id: T, cursor: Option<String>) -> Self {
        Self {
            q: None,
            count: Some(100),
            query_source: None,
            pc: None,
            spelling_corrections: None,
            cursor,
            user_id: Some(user_id.into()),
        }
    }
}

impl Default for AdaptiveParams {
    fn default() -> Self {
        Self::new()
    }
}
