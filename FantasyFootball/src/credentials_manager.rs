#[allow(unused)]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2
};

#[allow(unused)]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};

    
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims 
{
    pub sub: String,
    pub exp: i64
}
    
pub async fn create_token(secret: &str, user_id: &str) -> Result<String, jsonwebtoken::errors::Error>
{
    let claims = Claims {
        sub: user_id.to_string(),
        exp: chrono::Utc::now().timestamp() + (3600 * 2)
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub async fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error>
{
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )?;

    Ok(token_data.claims)
}

pub async fn password_hash(pwd: &str) -> Result<String, argon2::password_hash::Error>
{
    let salt = SaltString::generate(&mut OsRng);
    
    Ok(Argon2::default().hash_password(pwd.as_bytes(), &salt)?.to_string())
}
