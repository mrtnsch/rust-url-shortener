use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

use dotenv::dotenv;
use std::env;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
            .app_data(app_state.clone())
            .wrap(Logger::default())
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
