use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    q: String,
}

#[get("/suggestions")]
async fn suggestions(query: web::Query<Info>) -> impl Responder {
    let request_url = format!("https://nominatim.openstreetmap.org/search.php?q={}&accept-language=pt-br&countrycodes=br%2Cus%2Cca&limit=5&polygon_threshold=0&format=jsonv2", query.q);
    let res = reqwest::get(&request_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("Response Body: {}", res);
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(suggestions))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
