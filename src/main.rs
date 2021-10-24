#[macro_use]
extern crate rocket;

mod server;

#[rocket::main]
async fn main() {
    server::run().await
}
