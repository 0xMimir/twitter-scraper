use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GuestToken{
    pub guest_token: String
}