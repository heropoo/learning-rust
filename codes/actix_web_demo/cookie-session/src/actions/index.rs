use crate::models::user::User;
use actix_session::Session;
use actix_web::{
    get, middleware::Logger, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    Result,
};

pub async fn index(req: HttpRequest, session: Session) -> Result<HttpResponse> {
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

    if let Some(login_user) = session.get::<User>("user")? {
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
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
