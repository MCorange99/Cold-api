use std::{self};
use actix_web::{
                self,
                get, 
                web, 
                App, 
                HttpServer, 
                Responder,
                middleware::{Logger}
            };
use env_logger::Env;

pub static PORT: u16 = 8080;

mod api;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // logger
            .wrap(Logger::new("%a %{User-Agent}i")) // logger
            // .route("/", web::get().to(|| async { "Hello World!" }))
            
            
            // Register endpoints here
            .service(api::ping::run)

    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
