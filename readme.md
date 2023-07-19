
init repo
`cargo init rust-url-shortener`

add dependencies
`cargo add dotenv sea-orm actix-web`

Create hello world actix server
```
//src/main.rs
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

post route that shortens url
```
#[post("/url")]
async fn shorten_url(params: web::Query<UrlParameters>) -> impl Responder {
    HttpResponse::Ok().body(params.original_url.to_owned())
}
```

* implement in-memory url map

* share via app state
* get route which resolves to original url
* Now we have a basic url shortener
* introduce persistence in db
* add seaorm, generate migrations
* replace in-memory map with migrations

