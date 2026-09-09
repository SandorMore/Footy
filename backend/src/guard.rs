use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status, State};
use crate::credentials_manager::{verify_token, Claims};
use crate::AppConfig;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = req.cookies();

        let Some(cookie) = jar.get_private("access_token") else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        let config = req.guard::<&State<AppConfig>>().await
            .expect("AppConfig managed state missing");

        match verify_token(cookie.value(), &config.jwt_secret).await {
            Ok(claims) => Outcome::Success(claims),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}