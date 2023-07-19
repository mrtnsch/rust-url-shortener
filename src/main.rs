use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::Deserialize;
use uuid::Uuid;
use dotenv::dotenv;
use std::env;


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

    let shortened_url: String = Uuid::new_v4().to_string().chars().take(8).collect();

    let response_body = format!("Your shortened URL is: http://{}/{}", &app_state.server, shortened_url);
    HttpResponse::Ok().body(response_body)
}

struct AppState {
    server: String
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //read env variables
    dotenv().ok();
    let host = env::var("HOST").expect("Host has to be defined in env variables");
    let port = env::var("PORT").expect("Port has to be defined in env variables");
    let server = env::var("SERVER").expect("Server has to be defined in env variables");
    let db_connection_string = env::var("DATABASE_URL").expect("Database URL has to be defined in env variables");

    //set up app state
    let app_state = web::Data::new(AppState {
        server
    });

    //activate logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("Debug"));

    //start actual server
    HttpServer::new(move || {
        //move is used so closure takes ownership of app_state
        App::new()
            .service(hello)
            .service(shorten_url)
            .app_data(app_state.clone())
            .wrap(Logger::default())
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
