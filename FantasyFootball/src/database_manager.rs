#[allow(unused)]
use sqlx::PgPool;

pub struct UserRow 
{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub currency_balance: i32
}

pub async fn find_user_by_username(pool: PgPool, username: &str) -> Result<Option<UserRow>, sqlx::Error>
{
    sqlx::query_as!(
        UserRow,
        "SELECT id, username, email, password_hash, currency_balance FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&pool)
    .await
}

pub async fn create_user(pool: PgPool, username: &str, email: &str, password_hash: &str) -> Result<i32, sqlx::Error>
{
    let rec = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
        username,
        email,
        password_hash
    )
    .fetch_one(&pool)
    .await?;

    Ok(rec.id)
}