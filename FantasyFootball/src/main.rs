use rocket;

pub mod credentials_manager;

#[get("/players")]
fn retrieve_players() -> &'static str
{
    "players"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![retrieve_players])
        .launch()
        .await;
}
