use actix_web::{HttpServer, App, web};
use actix_web::web::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct  EntityId {
    id: i64
}

#[derive(Clone)]
struct FinalUser {
    id: i64,
    user_name: String,
    full_name: String
}

struct AppState {
    users: std::sync::RwLock<Vec<FinalUser>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(
        AppState {
            users: std::sync::RwLock::new(vec![
                FinalUser {id: 1, user_name: "dave".to_string(), full_name: "Dave Choi".to_string()}
            ])
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/v1")
                .service(
                    web::resource("/user/{id}")
                        .route(web::get().to(get_user_name))
                )
            )
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

async fn get_user_name(app_data: web::Data<AppState>, params: Path<EntityId>) -> String {
    let users = app_data.users.read().unwrap();
    users
        .iter()
        .find(|usr| {
            usr.id == params.id
        })
        .unwrap().clone().user_name
}