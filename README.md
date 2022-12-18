# Twitter Scraper

Ported [twitter-scraper](https://github.com/n0madic/twitter-scraper) from go to rust library

You can use this library to get the text of any user's Tweets trivially.

## Installation

After cloning directory add library to `Cargo.toml`

```shell
git clone https://github.com/0xMimir/twitter-scraper
```


## Usage

### Get user tweets

```rust
use twitter_scraper::TwitterScraper;

#[tokio::main]
async fn main() {
    let client = TwitterScraper::new();
    let (tweets, cursor) = client.get_users_tweets("elonmusk", None).await.unwrap();
    for tweet in tweets{
        println!("{:#?}", tweet);
    }
    println!("Next page cursor: {:?}", cursor);
}

```

It appears you can ask for up to 50 tweets (limit ~3200 tweets).

### Search tweets by query standard operators

Tweets containing “web scraping“, filtering out retweets:

```rust
use twitter_scraper::TwitterScraper;

#[tokio::main]
async fn main() {
    let mut client = TwitterScraper::new();
    client.get_guest_token().await.unwrap();
    let (tweets, cursor) = client.search_tweets("web scraping", None).await.unwrap();
    for tweet in tweets{
        println!("{:#?}", tweet);
    }
    println!("Next page cursor: {:?}", cursor);
}

```

<!-- The search ends if we have 50 tweets. 

See [Rules and filtering](https://developer.twitter.com/en/docs/tweets/rules-and-filtering/overview/standard-operators) for build standard queries.


#### Set search mode

```golang
scraper.SetSearchMode(twitterscraper.SearchLatest)
```

Options:

* `twitterscraper.SearchTop` - default mode
* `twitterscraper.SearchLatest` - live mode
* `twitterscraper.SearchPhotos` - image mode
* `twitterscraper.SearchVideos` - video mode
* `twitterscraper.SearchUsers` - user mode

### Get profile

```golang
package main

import (
    "fmt"
    twitterscraper "github.com/n0madic/twitter-scraper"
)

func main() {
    scraper := twitterscraper.New()
    profile, err := scraper.GetProfile("Twitter")
    if err != nil {
        panic(err)
    }
    fmt.Printf("%+v\n", profile)
}
```

### Search profiles by query

```golang
package main

import (
    "context"
    "fmt"
    twitterscraper "github.com/n0madic/twitter-scraper"
)

func main() {
    scraper := twitterscraper.New().SetSearchMode(twitterscraper.SearchUsers)
    for profile := range scraper.SearchProfiles(context.Background(), "Twitter", 50) {
        if profile.Error != nil {
            panic(profile.Error)
        }
        fmt.Println(profile.Name)
    }
}
```

### Get trends

```golang
package main

import (
    "fmt"
    twitterscraper "github.com/n0madic/twitter-scraper"
)

func main() {
    scraper := twitterscraper.New()
    trends, err := scraper.GetTrends()
    if err != nil {
        panic(err)
    }
    fmt.Println(trends)
}
```

### Use cookie authentication

Some specified user tweets are protected that you must login and follow.
Cookie and xCsrfToken is optional.

```golang
scraper.WithCookie("twitter cookie after login")
scraper.WithXCsrfToken("twitter X-Csrf-Token after login")
```

### Use Proxy

Support HTTP(s) and SOCKS5 proxy

#### with HTTP

```golang
err := scraper.SetProxy("http://localhost:3128")
if err != nil {
    panic(err)
}
```

#### with SOCKS5

```golang
err := scraper.SetProxy("socks5://localhost:1080")
if err != nil {
    panic(err)
}
```

### Delay requests

Add delay between API requests (in seconds)

```golang
scraper.WithDelay(5)
```

### Load timeline with tweet replies

```golang
scraper.WithReplies(true)
```
-->