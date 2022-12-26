#[tokio::test]
async fn test_search() {
    use crate::TwitterScraper;
    let scraper = TwitterScraper::new();
    scraper.get_guest_token().await.unwrap();
    let (_, cursor) = scraper.search("bitcoin", None).await.unwrap();
    assert!(cursor.is_some());
    let (tweets, cursor) = scraper.search("bitcoin", cursor).await.unwrap();
    assert!(tweets.len() > 0);
    assert!(cursor.is_some());
}
