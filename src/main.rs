#![deny(clippy::all, clippy::pedantic)]

use serde::Deserialize;
use warp::Filter;

#[tokio::main]
async fn main() {
    let lucky_numbers_route = warp::path("lucky_numbers")
        .and(warp::query::<LuckyNumbersRequest>())
        .map(|request: LuckyNumbersRequest| format!("{}", count_lucky_numbers(&request)));

    warp::serve(lucky_numbers_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn count_lucky_numbers(request: &LuckyNumbersRequest) -> usize {
    let sequence_string = request.sequence.to_string();
    (request.start..=request.end).fold(0, |accumulator, number| {
        accumulator + number.to_string().matches(&sequence_string).count()
    })
}

#[derive(Deserialize)]
struct LuckyNumbersRequest {
    start: i32,
    end: i32,
    sequence: i32,
}
