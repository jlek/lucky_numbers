#![deny(clippy::all, clippy::pedantic)]

use count_lucky_numbers::count_lucky_numbers;
use serde::Deserialize;
use warp::Filter;

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

#[derive(Deserialize)]
pub struct CountLuckyNumbersRequest {
    pub start: u32,
    pub end: u32,
    pub sequence: u32,
}
