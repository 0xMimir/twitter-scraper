use serde::Serialize;

use crate::{Error, Result};

pub struct Params {
    variables: Variables,
    features: Features,
}

#[derive(Serialize)]
struct ParamsUrl {
    variables: String,
    features: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Variables {
    user_id: String,
    count: i32,
    cursor: Option<String>,
    include_promoted_content: bool,
    with_super_follows_user_fields: bool,
    with_downvote_perspective: bool,
    with_reactions_metadata: bool,
    with_reactions_perspective: bool,
    with_super_follows_tweet_fields: bool,
}

#[derive(Serialize)]
struct Features {
    responsive_web_twitter_blue_verified_badge_is_enabled: bool,
    verified_phone_label_enabled: bool,
    responsive_web_graphql_timeline_navigation_enabled: bool,
    view_counts_public_visibility_enabled: bool,
    view_counts_everywhere_api_enabled: bool,
    tweetypie_unmention_optimization_enabled: bool,
    responsive_web_uc_gql_enabled: bool,
    vibe_api_enabled: bool,
    responsive_web_edit_tweet_api_enabled: bool,
    graphql_is_translatable_rweb_tweet_is_translatable_enabled: bool,
    standardized_nudges_misinfo: bool,
    tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled: bool,
    interactive_text_enabled: bool,
    responsive_web_text_conversations_enabled: bool,
    responsive_web_enhance_cards_enabled: bool,
}

impl Variables {
    pub fn new<T: Into<String>>(user_id: T, cursor: Option<T>) -> Self {
        Self {
            user_id: user_id.into(),
            count: 20,
            cursor: cursor.map(|c| c.into()),
            include_promoted_content: false,
            with_super_follows_user_fields: true,
            with_downvote_perspective: false,
            with_reactions_metadata: false,
            with_reactions_perspective: false,
            with_super_follows_tweet_fields: true,
        }
    }
}

impl Features {
    pub fn new() -> Self {
        Self {
            responsive_web_twitter_blue_verified_badge_is_enabled: true,
            verified_phone_label_enabled: false,
            responsive_web_graphql_timeline_navigation_enabled: true,
            view_counts_public_visibility_enabled: true,
            view_counts_everywhere_api_enabled: true,
            tweetypie_unmention_optimization_enabled: true,
            responsive_web_uc_gql_enabled: true,
            vibe_api_enabled: true,
            responsive_web_edit_tweet_api_enabled: true,
            graphql_is_translatable_rweb_tweet_is_translatable_enabled: true,
            standardized_nudges_misinfo: true,
            tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled: false,
            interactive_text_enabled: true,
            responsive_web_text_conversations_enabled: false,
            responsive_web_enhance_cards_enabled: true,
        }
    }
}

impl Default for Features {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Params> for ParamsUrl {
    type Error = Error;

    fn try_from(value: Params) -> Result<Self> {
        let features = serde_json::to_string(&value.features)?;
        let variables = serde_json::to_string(&value.variables)?;

        Ok(Self {
            features,
            variables,
        })
    }
}

impl Params {
    pub fn new<T: Into<String>>(user_id: T, cursor: Option<T>) -> Self {
        Self {
            variables: Variables::new(user_id, cursor),
            features: Features::default(),
        }
    }
    pub fn to_url(self) -> Result<String> {
        let url_params: ParamsUrl = self.try_into()?;
        serde_url_params::to_string(&url_params).map_err(Error::from)
    }
    pub fn add_currsor<T: Into<String>>(mut self, cursor: T) -> Self{
        self.variables.cursor = Some(cursor.into());
        self
    }
}

#[tokio::test]
async fn test_valid_serialization() {
    let test = Params::new("627673190", None).to_url().unwrap();
    let valid = "variables=%7B%22userId%22%3A%22627673190%22%2C%22count%22%3A20%2C%22includePromotedContent%22%3Afalse%2C%22withSuperFollowsUserFields%22%3Atrue%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withSuperFollowsTweetFields%22%3Atrue%7D&features=%7B%22responsive_web_twitter_blue_verified_badge_is_enabled%22%3Atrue%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%2C%22view_counts_public_visibility_enabled%22%3Atrue%2C%22view_counts_everywhere_api_enabled%22%3Atrue%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22responsive_web_uc_gql_enabled%22%3Atrue%2C%22vibe_api_enabled%22%3Atrue%2C%22responsive_web_edit_tweet_api_enabled%22%3Atrue%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Atrue%2C%22standardized_nudges_misinfo%22%3Atrue%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Afalse%2C%22interactive_text_enabled%22%3Atrue%2C%22responsive_web_text_conversations_enabled%22%3Afalse%2C%22responsive_web_enhance_cards_enabled%22%3Atrue%7D";
    assert_eq!(test, valid);
}
