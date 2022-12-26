use serde::Serialize;

#[derive(Serialize)]
pub struct AdaptiveParams {
    pub q: String,
    pub count: i32,
    pub query_source: String,
    pub pc: i32,
    pub spelling_corrections: i32,
    pub cursor: Option<String>
}

impl AdaptiveParams {
    pub fn search_params<T: Into<String>>(q: T, cursor: Option<String>) -> Self {
        Self {
            q: q.into(),
            count: 10,
            query_source: "typed_query".to_owned(),
            pc: 1,
            spelling_corrections: 1,
            cursor
        }
    }
    pub fn add_cursor(mut self, cursor: String) -> Self{
        self.cursor = Some(cursor);
        self
    }
}
