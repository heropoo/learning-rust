use actix_session::{CookieSession, Session};
use actix_web::{
    get, middleware::Logger, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    Result,
};
// use std::env;
use cookie_session::user;

async fn index(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    //log::info!("{:?}", req);

    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        log::info!("SESSION value: {}", count);
        counter = count + 1;
        session.insert("counter", counter)?;
    } else {
        session.insert("counter", counter)?;
    }

    if let Some(login_user) = session.get::<user::User>("user")? {
        Ok(HttpResponse::Ok().body(format!(
            "welcome {}, {} times!",
            serde_json::to_string(&login_user)?,
            counter
        )))
    } else {
        Ok(HttpResponse::Ok().body(format!("welcome {} times!", counter)))
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

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
            .service(web::resource("/").to(index))
            .service(user::login)
            .service(user::logout)
            .service(echo)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
