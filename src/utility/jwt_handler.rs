use chrono::Utc;
use jsonwebtoken::{decode, encode, errors::Result, Algorithm, DecodingKey, EncodingKey, Header};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct JwtHandler {
    pub private_key: Secret<String>,
    pub header: Header,
    pub public_key: String,
    pub expiration_minutes: i64,
}

impl JwtHandler {
    pub fn create_token(self, user_name: &str) -> String {
        let claims = Claims {
            aud: user_name.to_owned(),
            exp: (Utc::now() + chrono::Duration::minutes(self.expiration_minutes)).timestamp()
                as usize,
            iat: Utc::now().timestamp() as usize,
            iss: "zero_to_production".to_owned(),
            nbf: Utc::now().timestamp() as usize,
            sub: "zero_to_production".to_owned(),
        };

        encode(
            &self.header,
            &claims,
            &EncodingKey::from_secret(self.private_key.expose_secret().as_ref()),
        )
        .unwrap_or_default()
    }

    pub fn decode_token(self, token: String) -> Result<Claims> {
        let key = self.private_key.expose_secret().as_ref();
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(key),
            &jsonwebtoken::Validation::new(Algorithm::HS512),
        )
        .map(|data| data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience (who or what the token is intended for)
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp) (default = now)
    iss: String, // Optional. Issuer (who issued the token)
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}
