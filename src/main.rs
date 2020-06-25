#![deny(clippy::all, clippy::pedantic)]

use serde::Deserialize;
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

#[allow(clippy::needless_pass_by_value)] // Accept the request by value: the request is consumed to produce the response
fn count_lucky_numbers(request: LuckyNumbersRequest) -> String {
    let sequence_string = request.sequence.to_string();
    (request.start..=request.end)
        .fold(0, |accumulator, number| {
            accumulator + number.to_string().matches(&sequence_string).count()
        })
        .to_string()
}

#[derive(Deserialize)]
struct LuckyNumbersRequest {
    start: i32,
    end: i32,
    sequence: i32,
}
