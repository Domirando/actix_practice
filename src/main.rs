use actix_web::{get, post, HttpResponse, web, web::get, App, HttpServer, Responder};
use serde::Deserialize;

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(|| async {HttpResponse::Ok().body("get")}))
            .service(hello)

    })

.bind("0.0.0.0:3000")
.unwrap()
    .run()
    .await
    .unwrap()
    //println!("Hello, world!");
}

#[post("/user")]
async fn hello(info: web::Json<Info>) -> impl Responder {
    let msg = format!("name: {}, age: {}", info.name, info.age);
    HttpResponse::Ok().body(msg)
    }

#[derive(Deserialize)]
struct Info{
    name: String,
    age: i32
    }

