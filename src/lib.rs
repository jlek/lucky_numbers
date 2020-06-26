use serde::Deserialize;

#[derive(Deserialize)]
pub struct LuckyNumbersRequest {
  pub start: i32,
  pub end: i32,
  pub sequence: i32,
}

#[allow(clippy::needless_pass_by_value)] // Accept the request by value: the request is consumed to produce the response
pub fn count_lucky_numbers(request: LuckyNumbersRequest) -> String {
  let sequence_string = request.sequence.to_string();
  (request.start..=request.end)
    .fold(0, |accumulator, number| {
      accumulator + number.to_string().matches(&sequence_string).count()
    })
    .to_string()
}
