# Overview
* In this project, we implemented an URL-shortening service using Rust and the Actix Web framework
* We will start of simple and gradually extend the service with additional functionality
* Note: This is an educational project and probably is not a good implementation of an URL-shortening service.

## Steps
1. Initialize a new rust project, adding actix-web, simple hello world
2. Implementing of shortening route
3. Implementing app state, persisting routes in memory
4. Resolving shortened routes, redirecting
5. Adding persistence - SeaORM and our first migration
6. Creating entities from the schema, writing to the database
7. Cleanup & Refactoring

# Prerequisites
* Rust toolchain. The easiest way is to use [rustup](https://rustup.rs/)
  * Rust needs a linker. You might want to install a C-compiler since they usually include one. On macOS, you can run `xcode-select --install` 
* An IDE with Rust plugins (I use IntelliJ IDEA with the official Rust plugin)
* Docker compose for our postgres database

# Building the rust-url-shortener
## Step 1: Initializing the project

To get started, we use cargo to initialize the project:
`cargo init rust-url-shortener`

* This command will create a basic folder to get started.
* The entrypoint to our app will be the main.rs created by cargo.
* The Cargo.toml contains metadata and information about our dependencies (similar to a package.json or a pom.xml)
* For this project, we will use the [Actix Web](https://actix.rs/docs/) framework
* To get started, we add the necessary dependency to our Cargo.toml:
```
[dependencies]
actix-web = "4"
```
* A basic hello world example can be found in the Actix docs under [Getting started](https://actix.rs/docs/getting-started)
* Replace the contents of `src/main.rs` with this code
```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
* This is sufficient to get a server up and running


## Step 2: Implementing the shortening route
* Before proceeding, I recommend to checkout the branch for this step. The code will include some tasks like reading .env variables.
* This route should be a POST-request to `/shorten`
* The body should be x-www-form-urlencoded and contain the key `original_url` and the value should be the url we want to shorten, e.g. `https://www.google.com`
* The response should return the shortened url to the client
* I recommend having a look at [this section](https://actix.rs/docs/extractors#url-encoded-forms) of the Actix docs

## Step 3: Saving the shortened routes in-memory
* As intermediate solution, we want to keep the shortened routes in memory using a hashmap
* Some remarks:
  * To make the hashmap available to all worker threads, we can share it via Actix's AppState
  * The threads need mutable access to the hashmap. This can be achieved by wrapping the hashmap in a mutex
  * Refer to [this section](https://actix.rs/docs/application#shared-mutable-state) for further details.

## Step 4: Resolving the shortened routes
* Now we can resolve the shortened routes and redirect the client to the stored target url
* The routes should be called via a GET-request to `/{shorturl}`, where `{shorturl}` is a path parameter
* Refer to [this section](https://actix.rs/docs/extractors#path) for details
* You will need to read the corresponding entry of the hashmap from the app state
* Getting from a hashmap in Rust will return an Option<T>. The proper way to handle them is using Rust's `match`-construct
* For redirecting, you will want to use something like:
```
    HttpResponse::TemporaryRedirect()
      .insert_header(("Location", "target_url"))
      .finish(),
```

## Step 5: Adding persistence
* For persistence, we use a Postgres db with SeaORM. You can use the provided docker-compose to start a db
* To get started, add the dependency to the Cargo.toml
  * `sea-orm = { version = "^0", features = [ "sqlx-postgres", "runtime-actix-native-tls", "macros" ] }`
* Furthermore, we install the sea-orm cli to create the migrations
  * `cargo install sea-orm-cli`
* Initialize the migrations using
  * `sea-orm-cli migrate init`
* Now it is time to reorganize our folder structure and their Cargo.toml files. Follow the description in  https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/
* To create the actual migration, run
  * `sea-orm-cli migrate generate create_url_map_table` to create an empty migration to start off with
* Open the freshly created migration file and update its the up function to the following:
```rust
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                        .table(UrlStore::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(UrlStore::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UrlStore::ShortUrl)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(ColumnDef::new(UrlStore::TargetUrl).string().not_null())
                    .col(ColumnDef::new(UrlStore::CreatedAt).timestamp().not_null().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)))
                    .to_owned()
            )
            .await
    }
```
* Furthermore, you can delete the unnecessary migration which was created by using the CLI
  * As additional preparation, we create an entity crate which will hold our entities.
    * You can run `cargo new entity --lib` to do so
    * Next, add sea-orm to the dependencies of the `entity/Cargo.toml`
    ```
      [dependencies]
      sea-orm = { version = "^0" }
    ```
    * Additionally, we need some additional setup in our main Cargo.toml. Make sure it contains the following:
    ```
    [workspace]
    members = [".", "entity", "migration"]
  
    [dependencies]
    entity = { path = "entity" }
    migration = { path = "migration" } # depends on your needs
  
    [dependencies.sea-orm]
    version = "^0"
    features = [ ... ]
    ```
* Next, we need to tell our server how to connect to our database. Furthermore, the migrations should be run on startup. Add the following code to the main function:
```rust

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
```
* Now run the server. Inspect your database and make sure the migrations have successfully run.

## Step 6: Generating the entities from the schema
* We can use the sea-orm-cli to generate entities from our database schema
  * `sea-orm-cli generate entity -o entity/src`
* This will create the file url_store.rs in the folder entity/src, which we can use to talk to the database
* For further instructions on creating records in the database and reading them, refer to the [seaORM documentation](https://www.sea-ql.org/SeaORM/docs/basic-crud/insert/#insert-one)
* The task now is to replace the in-memory hashmap with the db connection
* To use the db connection in the route handlers, you can add it to AppState


## Step 7: Refactoring
* The final version might have some minor changes and reorganization in it

