pub mod error;
pub use error::{Result, Error};

pub mod types;

pub mod scraper;
pub use scraper::TwitterScraper;

pub mod search;