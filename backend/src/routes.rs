use rocket::serde::json::Json;
use rocket::http::{Cookie, CookieJar, Status, SameSite};
use rocket::State;
use rocket::time::Duration;
use serde::Deserialize;

use crate::credentials_manager::{create_token, password_hash, verify_password, Claims};
use crate::database_manager::{create_user, find_user_by_username};
use crate::AppConfig;
use crate::username_validator::{validate_username};

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
    pub email: String
}

#[post("/register", data = "<req>")]
pub async fn register(req: Json<AuthRequest>, config: &State<AppConfig>) -> Result<Status, Status> {
    let hash = password_hash(&req.password).await.map_err(|_| Status::InternalServerError)?;
    create_user(config.pool.clone(), &req.username, &req.email, &hash).await.map_err(|_| Status::Conflict)?;
    Ok(Status::Created)
}

#[post("/login", data = "<req>")]
pub async fn login(req: Json<AuthRequest>, config: &State<AppConfig>, jar: &CookieJar<'_>, ) -> Result<Status, Status> {
    let user = find_user_by_username(config.pool.clone(), &req.username)
        .await.map_err(|_| Status::InternalServerError)?
        .ok_or(Status::Unauthorized)?;

    let valid = verify_password(&req.password, &user.password_hash)
        .await
        .map_err(|_| Status::InternalServerError)?;

    if !valid {
        return Err(Status::Unauthorized);
    }

    let token = create_token(&config.jwt_secret, &user.id.to_string())
        .await.map_err(|_| Status::InternalServerError)?;

    jar.add_private(
        Cookie::build(("access_token", token))
            .same_site(SameSite::Lax)
            .max_age(Duration::hours(2))
    );

    Ok(Status::Ok)
}

#[get("/players")]
pub async fn retrieve_players(claims: Claims) -> String {
    format!("players — hello user {}", claims.sub)
}

#[get("/")]
pub async fn startup_page() -> &'static str
{
    "hello"
} 