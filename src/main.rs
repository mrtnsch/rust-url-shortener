use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::Deserialize;
use uuid::Uuid;
use dotenv::dotenv;
use std::env;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct UrlInfoBody {
    original_url: String,
}

#[post("/shorten")]
async fn shorten_url(body: web::Form<UrlInfoBody>, app_state: web::Data<AppState>) -> impl Responder {
    let server = &app_state.server;

    let shortened_url = generate_short_text();
    let mut url_map = app_state.url_map.lock().unwrap();
    url_map.insert(shortened_url.to_string(), body.original_url.to_owned());

    let response_body = format!("Your shortened URL is: http://{}/{}", server, shortened_url);
    HttpResponse::Ok().body(response_body)
}

#[get("/{shorturl}")]
async fn resolve_shortened_url(short_url: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let url_map = app_state.url_map.lock().unwrap();
    let result = url_map.get(&short_url.into_inner());

    match result {
        Some(target_url) => HttpResponse::TemporaryRedirect()
            .insert_header(("Location", target_url.to_string()))
            .finish(),
        None => HttpResponse::NotFound().body("URL not found"),
    }

}

fn generate_short_text() -> String {
    Uuid::new_v4().to_string().chars().take(8).collect()
}

struct AppState {
    url_map: Mutex<HashMap<String, String>>,
    server: String,
    db: DatabaseConnection
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //read env variables
    dotenv().ok();
    let host = env::var("HOST").expect("Host has to be defined in env variables");
    let port = env::var("PORT").expect("Port has to be defined in env variables");
    let db_connection_string = env::var("DATABASE_URL").expect("Database URL has to be defined in env variables");


    //set up db connection
    let mut database_connection_options = ConnectOptions::new(db_connection_string);
    database_connection_options.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level( log::LevelFilter::Info)
        .set_schema_search_path("public".into());
    let db = Database::connect(database_connection_options).await.expect("unable to connect to the database");

    //run migrations
    Migrator::up(&db, None).await.expect("An error occurred while running the migrations");

    //set up app state
    let app_state = web::Data::new(AppState {
        url_map: Mutex::new(HashMap::new()),
        server: env::var("SERVER").expect("Server has to be defined in env variables"),
        db: db
    });

    //activate logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("Debug"));

    //start actual server
    HttpServer::new(move || {
        //move is used so closure takes ownership of app_state
        App::new()
            .service(hello)
            .service(shorten_url)
            .service(resolve_shortened_url)
            .app_data(app_state.clone())
            .wrap(Logger::default())
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
