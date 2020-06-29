use rayon::prelude::*;
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
  let sequence_bytes = sequence_string.as_bytes();
  (request.start..=request.end)
    .into_par_iter()
    .fold(
      || 0,
      |accumulator, number| {
        accumulator + number_of_matches(number.to_string().as_bytes(), sequence_bytes)
      },
    )
    .sum::<usize>()
    .to_string()
}

pub fn number_of_matches(number: &[u8], sequence: &[u8]) -> usize {
  let mut result = 0;

  // If the sequence does not "fit" in the number, there can be no matches
  if sequence.len() > number.len() {
    return 0;
  }

  for i in 0..number.len() - sequence.len() + 1 {
    let num_char = number[i];
    let seq_char = sequence[0];

    if num_char == seq_char {
      for j in 1..sequence.len() {
        let num_char = number[i + j];
        let seq_char = sequence[j];
        if num_char != seq_char {
          break;
        }
        if j == (sequence.len() - 1) {
          result += 1;
        }
      }
    }
  }

  result
}

#[test]
fn test_count_lucky_numbers() {
  // Arrange
  let request = LuckyNumbersRequest {
    start: 100,
    end: 150,
    sequence: 14,
  };

  // Act
  let result = count_lucky_numbers(request);

  // Assert
  assert_eq!(result, "11".to_string()); // 114, 140 ... 149
}

#[test]
fn test_count_lucky_numbers_large() {
  // Arrange
  let request = LuckyNumbersRequest {
    start: 0,
    end: 1000,
    sequence: 123,
  };

  // Act
  let result = count_lucky_numbers(request);

  // Assert
  assert_eq!(result, "1".to_string());
}
