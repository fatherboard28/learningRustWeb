use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello"
}

async fn lol() -> impl Responder {
    "read me daddy"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app").route("/index.html", web::get().to(index)),)
            .service(
                web::scope("/daddy").route("/index2.html", web::get().to(lol)),)
            //.app_data(web::Data::new(AppState {
            //  app_name: String::from("Bob sagit")
            //}))
            //.service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
