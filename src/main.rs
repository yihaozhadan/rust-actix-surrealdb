mod api;
mod db;
mod error;
mod model;
mod prelude;

use once_cell::sync::Lazy;
// use actix_cors::Cors;
use actix_web::{App, HttpServer};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use api::*;
static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //const DB: Surreal<Client> = Surreal::init();
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("todo").use_db("todo").await?;

    println!("✅ Database connected successfully!!");

    println!("✅ Server running at http://localhost:{PORT}");

    HttpServer::new(|| {
        // let cors = Cors::default()
        //     .allow_any_origin()
        //     .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        //     .send_wildcard();

        App::new()
            //.wrap(cors)
            .service(create)
            .service(get)
            .service(update)
            .service(delete)
            .service(list)
    })
    .bind(("localhost", PORT))?
    .run()
    .await?;

    Ok(())
}