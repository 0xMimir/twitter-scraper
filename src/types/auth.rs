use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GuestToken{
    pub guest_token: String
}

pub struct CSRFAuth{
    pub auth_token: String,
    pub csrf_token: String
}