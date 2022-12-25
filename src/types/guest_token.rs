use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GuestToken{
    pub guest_token: String
}