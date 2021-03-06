use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};

// use serde_json;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[derive(Serialize, Deserialize)]
struct User{
    id: u32,
    username: String,
    password: String,
    created_at: u64,
    updated_at: u64
}

fn user() -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().json(User {
        id: 1,
        username: String::from("小明"),
        password: String::from("123456"),
        created_at: 12312,
        updated_at: 1232134343
    }))
}

/// # actix-web
/// actix-web-demo
/// 啦啦啦啦啦啦
/// - actix-web
/// end
fn main() {
    println!("Listenning on http://127.0.0.1:8088");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/user", web::get().to(user))
            .route("/hello/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
