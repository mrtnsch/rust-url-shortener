
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
* add seaorm
* `sea-orm = { version = "^0", features = [ <sqlx-postgres>, <runtime-actix-native-tls>, "macros" ] }`
* prepare for running migrations
`cargo install sea-orm-cli`
`sea-orm-cli migrate init`
* Reorganize tomls as described in https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/
* write the actual migrations
* `sea-orm-cli migrate generate create_url_map_table` to create an empty migration to start off with
* Write seaorm ddl
* Run server, verify the migration was run

### Generating the entities


* replace in-memory map with migrations

