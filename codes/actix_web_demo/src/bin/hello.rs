use actix_web::{get, web, App, HttpServer, HttpResponse,Responder};

#[get("/{id}/{name}/index.html")]
async fn index(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://127.0.0.1:8000/");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
        }).bind(("127.0.0.1", 8000))?
        .run()
        .await
}
