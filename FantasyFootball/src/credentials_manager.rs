use argon2::PasswordVerifier;
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
    
pub struct RefreshToken 
{
    pub token_hash: String,
    pub user_id: String,
    pub expires_at: i64,
    pub revoked: bool

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

pub async fn verify_password(pwd: &str, hash: &str) -> Result<bool, argon2::password_hash::errors::Error> 
{
    let parse_hash = argon2::PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(pwd.as_bytes(), &parse_hash).is_ok())
}

//unit tests
#[cfg(test)]
mod tests 
{
    use super::*;
    
    #[tokio::test]
    async fn password_hash_can_be_verified() {
        let hash = password_hash("secret").await.unwrap();

        assert!(verify_password("secret", &hash).await.unwrap());
        assert!(!verify_password("wrong", &hash).await.unwrap());
    }

    #[tokio::test]
    async fn token_round_trip_works() {
        let token = create_token("test-secret", "42").await.unwrap();
        let claims = verify_token(&token, "test-secret").await.unwrap();

        assert_eq!(claims.sub, "42");
    }

    #[tokio::test]
    async fn token_with_wrong_secret_fails() {
        let token = create_token("test-secret", "42").await.unwrap();

        assert!(verify_token(&token, "wrong-secret").await.is_err());
    }
}