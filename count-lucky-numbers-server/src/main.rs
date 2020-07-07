#![deny(clippy::all, clippy::pedantic)]

//! This binary crate starts a web server with an endpoint that wraps the `count_lucky_numbers` function.

use count_lucky_numbers::count_lucky_numbers;
use serde::Deserialize;
use warp::Filter;

/// The entry point of the application. Starts a web server with an endpoint that wraps the `count_lucky_numbers` function.
#[tokio::main]
async fn main() {
    let lucky_numbers_route = warp::path("lucky_numbers")
        .and(warp::query::<CountLuckyNumbersRequest>())
        .map(|request: CountLuckyNumbersRequest| {
            count_lucky_numbers(request.start, request.end, request.sequence).to_string()
        });

    warp::serve(lucky_numbers_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
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
    start: u32,
    end: u32,
    sequence: u32,
}
