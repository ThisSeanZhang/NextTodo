use salvo::{affix, prelude::TcpListener, Router};
use service::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use std::env;
use salvo::Server;
use salvo::Listener;

mod task_api;
mod agenda_api;
mod task_relate;
mod graph_position;

#[tokio::main]
pub async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let webroot = env::var("WEBROOT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // create post table if not exists
    let conn = Database::connect(&db_url).await.unwrap();
    // Cancle Auto Run
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn };

    println!("Starting server at {server_url}");

    let router = Router::new()
    .push(
        Router::new()
        .hoop(affix::inject(state))
        .push(task_api::get_route())
        .push(agenda_api::get_route())
        .push(task_relate::get_route())
        .push(graph_position::get_route())
    )
    .push(Router::with_path("<**path>").get(
        salvo::prelude::StaticDir::new(webroot)
        .defaults("index.html")
    ));

    Server::new(TcpListener::bind(TcpListener::new(format!("{host}:{port}"))).await)
        .serve(router)
        .await;
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

