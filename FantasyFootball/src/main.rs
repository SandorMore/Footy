#[allow(unused)]
#[macro_use] extern crate rocket;

#[get("/hello")]
fn hello() -> &'static str
{
    "hello"
}


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await;
}
