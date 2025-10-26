use actix_web::{HttpResponse, web, App, HttpServer};

#[actix_web::main]
async fn main() {
    HttpServer::new(||
        App::new()
            .route("/", web::get().to(|| async {
            HttpResponse::Ok().body("hello world".to_string())
            })))
.bind("0.0.0.0:3000")
.unwrap()
    .run()
    .await
    .unwrap()
    //println!("Hello, world!");
}
