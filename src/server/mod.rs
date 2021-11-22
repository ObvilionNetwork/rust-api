pub mod routes;

pub async fn run() {
    let _ = rocket::build()
        .mount("/api", routes![routes::index::index, routes::files::get])
        .launch()
        .await;
}
