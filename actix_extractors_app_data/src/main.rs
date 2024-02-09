use std::sync::Mutex;
use actix_web::{HttpServer, App};

#[derive(Clone)]
struct Messenger {
    message: String
}

#[derive(Clone)]
struct appState {
    messenger: Messenger
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = appState {
        messenger: Messenger { message: "Oi".to_owned()}
    };

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(app_data.clone()))
            .route("/", actix_web::web::post().to(update))
            .route("/", actix_web::web::get().to(get))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

async fn update(app_data: actix_web::web::Data<appState>) -> String {
    let messenger = app_data.messenger.clone();
    format!("{} world", messenger.message)
}

async fn get(app_data: actix_web::web::Data<appState>) -> String {
    app_data.messenger.message.clone()
}
