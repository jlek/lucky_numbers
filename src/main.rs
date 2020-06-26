#![deny(clippy::all, clippy::pedantic)]

use lucky_numbers::{count_lucky_numbers, LuckyNumbersRequest};
use warp::Filter;

#[tokio::main]
async fn main() {
    let lucky_numbers_route = warp::path("lucky_numbers")
        .and(warp::query::<LuckyNumbersRequest>())
        .map(count_lucky_numbers);

    warp::serve(lucky_numbers_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
