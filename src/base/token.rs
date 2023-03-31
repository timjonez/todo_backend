use chrono::prelude::Utc;
use chrono::Duration;
use dotenv::dotenv;
use email_address::EmailAddress;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub email: EmailAddress,
    pub exp: i64,
}

impl Claims {
    pub fn new(id: String, email: EmailAddress) -> Self {
        let exp = Utc::now() + Duration::days(1);
        Claims {
            id,
            email,
            exp: exp.timestamp(),
        }
    }
}

pub fn get_token(id: String, email: EmailAddress) -> String {
    dotenv().ok();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    let payload = Claims::new(id, email);
    encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn check_token(token: String) -> Result<TokenData<Claims>, Error> {
    dotenv().ok();
    let secret = dotenv::var("SECRET_KEY").unwrap();
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}
