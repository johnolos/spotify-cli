#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i32,
    pub refresh_token: String,
}

pub struct AuthorizationCode {
    pub code: String
}

impl AuthorizationCode {
    pub fn new(code: String) -> AuthorizationCode {
        AuthorizationCode { code }
    }
}

pub struct Credentials {
    pub client_id: String,
    pub secret: String,
    pub code: String,
}

impl Credentials {
    pub fn new(client_id: String, secret: String, code: String) -> Credentials {
        Credentials {
            client_id,
            secret,
            code,
        }
    }
}
