mod error;
pub use crate::error::{Error, CResult as Result}; 

pub mod store;
pub mod scraper;
pub use scraper::TwitterScraper;