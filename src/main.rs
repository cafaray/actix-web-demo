use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[put("/")]
async fn update(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn remove() -> impl Responder {
    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(update)
            .route("/", web::delete().to(remove))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}