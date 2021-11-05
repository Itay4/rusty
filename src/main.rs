use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

async fn get_health_status(data: web::Data<AppState>) -> HttpResponse {
    let is_database_connected = sqlx::query("SELECT 1")
        .fetch_one(&data.db_conn)
        .await
        .is_ok();
    if is_database_connected {
        HttpResponse::Ok()
            .content_type("application/json")
            .body("Ok")
    } else {
        HttpResponse::ServiceUnavailable()
            .content_type("application/json")
            .body("Bad")
    }
}

struct AppState {
    db_conn: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var is not set");
    
    let db_conn = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(2))
        .connect(database_url.as_str()) // <- Use the str version of database_url variable.
        .await
        .expect("failed connecting to db");

    let app_state = web::Data::new(AppState {
        db_conn: db_conn
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/health", web::get().to(get_health_status))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}