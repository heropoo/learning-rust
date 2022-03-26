use actix_session::CookieSession;
use actix_web::{
    middleware::Logger, web, App, HttpServer
};
// use std::env;
use cookie_session::actions::{index, user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(Logger::default())
            //cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(web::resource("/").to(index::index))
            .service(user::login)
            .service(user::logout)
            .service(index::echo)
            .service(index::hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
