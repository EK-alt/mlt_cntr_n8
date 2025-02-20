// use mysql::{prelude::Queryable, Pool};
// use std::env;
// fn main() {
//     println!("Hello, world from Rust!");
//     // let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
//     // let pool = Pool::new(DATABASE_URL.as_str()).expect("Failed to create a pool");
//     // let mut conn = pool.get_conn().expect("Failed to get connection.");
//     // let result: Vec<String> = conn
//     //     // .query("SELECT CURRENT_TIMESTAMP() AS now")
//     //     .query("SELECT DATE_FORMAT(UTC_TIMESTAMP(6), '%Y-%m-%dT%H:%i:%s.%fZ') AS now")
//     //     .expect("Query failed");

//     // println!("aRes {:?} ", result);
// }

use actix_web::{web, App, HttpServer, Responder};

use mysql::{prelude::Queryable, Pool};
use std::env;

async fn hello() -> impl Responder {
    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let pool = Pool::new(DATABASE_URL.as_str()).expect("Failed to create a pool");
    let mut conn = pool.get_conn().expect("Failed to get connection.");
    let result: Vec<String> = conn
        // .query("SELECT CURRENT_TIMESTAMP() AS now")
        .query("SELECT DATE_FORMAT(UTC_TIMESTAMP(6), '%Y-%m-%dT%H:%i:%s.%fZ') AS now")
        .expect("Query failed");

    println!("aRes {:?} ", result);
    let result = result[0].clone();
    format!("Hello, world! and response from MySQL {}", result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world from Rust!");
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello)) // Route for /hello
            .route("/", web::get().to(hello)) // Route for / (root path) - optional
    })
    .bind(("0.0.0.0", 9093))? // Bind to all interfaces (0.0.0.0) and port 9090
    .run()
    .await
}
