#![deny(clippy::all, clippy::pedantic)]

use serde::Deserialize;
use warp::Filter;

#[tokio::main]
async fn main() {
    let lucky_numbers_route = warp::path("lucky_numbers")
        .and(warp::query::<LuckyNumbersRequest>())
        .map(|request: LuckyNumbersRequest| format!("{}", request.sequence));

    warp::serve(lucky_numbers_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Deserialize)]
struct LuckyNumbersRequest {
    start: i32,
    end: i32,
    sequence: i32,
}
