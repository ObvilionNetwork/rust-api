pub mod routes;

pub async fn run() {
    let _ = rocket::build().mount("/", routes![routes::index::index]).launch().await;
}

