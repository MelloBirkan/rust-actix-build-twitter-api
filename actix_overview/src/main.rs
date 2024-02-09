use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            .service(
                actix_web::web::scope("/v1") // Para criar sub-rotas
                    .route("/hello", actix_web::web::get().to(index))
                    .route("/hello", actix_web::web::post().to(insert))
            )
            .service(
                actix_web::web::scope("/v2") // Para criar sub-rotas
                    .route("/2", actix_web::web::get().to(index))
                    .route("/2", actix_web::web::post().to(insert))
            )
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

async fn index() -> &'static str {
    "hello, world"
}

async fn insert() -> String {
    "Inserted".to_owned()
}