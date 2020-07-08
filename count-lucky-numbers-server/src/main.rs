#![deny(clippy::all, clippy::pedantic)]

//! This binary crate starts a web server with an endpoint that wraps the `count_lucky_numbers` function.

use actix_web::{web, App, HttpServer};
use count_lucky_numbers::count_lucky_numbers;
use serde::Deserialize;

/// The entry point of the application. Starts a web server with an endpoint that wraps the `count_lucky_numbers` function.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/lucky_numbers", web::get().to(lucky_numbers)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

async fn lucky_numbers(request: web::Query<CountLuckyNumbersRequest>) -> String {
    count_lucky_numbers(request.start, request.end, request.sequence).to_string()
}

/// Represents the query parameters required for a `lucky_numbers` request. This will get deserialised automaticaly by Warp/Serde.
///
/// # Examples
///
/// `?start=100&end=150&sequence=14` gets deserialised to:
///
/// ```no_run
/// CountLuckyNumbersRequest {
///     start: 100,
///     end: 150,
///     sequence: 14,
/// }
/// ```
#[derive(Deserialize)]
struct CountLuckyNumbersRequest {
    /// The first number in the range of numbers to consider
    start: u32,
    /// The last number in the range of numbers to consider
    end: u32,
    /// The total number of occurrences of this sequence in the range will be counted
    sequence: u32,
}
