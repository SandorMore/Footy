pub struct AppConfig 
{
    pub jwt_secret: String,
    pub pool: sqlx::PgPool
}
