#[allow(unused)]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2
};

#[allow(unused)]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serede::{Serialize, Deserialize};

    
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims 
{
    pub sub: String,
    pub exp: usize
}
    
pub fn create_token(secret: &str, user_id: &str) -> Result<String, jsonwebtoken::errors::Error>
{
    let claims = Claims {
        sub: user_id.to_string(),
        exp: 999999999
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.to_string()))
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error>
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
