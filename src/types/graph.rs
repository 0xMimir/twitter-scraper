use serde::Deserialize;

use crate::{profile::Profile, Error, Result};

use super::profile::TwitterUser;

#[derive(Deserialize, Debug)]
pub struct GraphResponse {
    data: GraphResponseData,
}

#[derive(Deserialize, Debug)]
struct GraphResponseData {
    user: GraphResponseUser,
}

#[derive(Deserialize, Debug)]
struct GraphResponseUser {
    result: GraphResponseResult,
}

#[derive(Deserialize, Debug)]
struct GraphResponseResult {
    __typename: String,
    timeline: Option<GraphResponseTimelineOuter>,
}

#[derive(Deserialize, Debug)]
struct GraphResponseTimelineOuter {
    timeline: GraphResponseTimeline,
}

#[derive(Deserialize, Debug)]
struct GraphResponseTimeline {
    instructions: Vec<GraphResponseInstructions>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum GraphResponseInstructions {
    Valid(ValidInstruction),
    Ignore(IgnoreInstruction),
}

#[derive(Deserialize, Debug)]
struct IgnoreInstruction {
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Deserialize, Debug)]
struct ValidInstruction {
    entries: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
struct Entry {
    content: ContentEnum,
}

#[derive(Deserialize, Debug)]
struct Content {
    #[serde(rename = "itemContent")]
    item_content: ItemContent,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ContentEnum {
    Cursor(Cursor),
    Item(Box<Content>),
}

#[derive(Deserialize, Debug)]
struct Cursor {
    __typename: String,
    value: String,
    #[serde(rename = "cursorType")]
    cursor_type: String,
}

#[derive(Deserialize, Debug)]
struct ItemContent {
    user_results: UserResults,
}

#[derive(Deserialize, Debug)]
struct UserResults {
    result: TwitterUser,
}

impl GraphResponse {
    pub fn get_users(self) -> Result<(Vec<Profile>, Option<String>)> {
        let mut profiles = vec![];
        let mut next_cursor = None;

        match self.data
            .user
            .result
            .timeline{
                Some(t) => t,
                None => return Err(Error::from(self.data.user.result))
            }
            .timeline
            .instructions
            .into_iter()
            .for_each(|x| {
                if let GraphResponseInstructions::Valid(valid) = x {
                    for entry in valid.entries {
                        match entry.content {
                            ContentEnum::Cursor(cursor) => {
                                if cursor.cursor_type == "Bottom" {
                                    next_cursor = Some(cursor.value);
                                }
                            }
                            ContentEnum::Item(item) => {
                                match Profile::try_from(item.item_content.user_results.result) {
                                    Ok(profile) => profiles.push(profile),
                                    _ => continue,
                                }
                            }
                        }
                    }
                }
            });

        Ok((profiles, next_cursor))
    }
}

impl From<GraphResponseResult> for Error{
    fn from(value: GraphResponseResult) -> Self {
        match value.__typename.as_str(){
            "UserUnavailable" => Self::UnauthorizedToViewSpecificUser,
            _ => Self::UnknownError
        }
    }
}