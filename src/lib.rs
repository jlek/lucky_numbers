use rayon::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LuckyNumbersRequest {
  pub start: i32,
  pub end: i32,
  pub sequence: i32,
}

struct LuckyNumbers {
  sequence: i32,
  previous_value: Option<i32>,
  order_of_magnitude: u32,
}

impl LuckyNumbers {
  #[allow(dead_code)]
  fn new(sequence: i32) -> Self {
    let sequence_length = (sequence as f64).log10().ceil() as usize;
    Self {
      sequence,
      previous_value: None,
      order_of_magnitude: sequence_length as u32,
    }
  }
}

impl Iterator for LuckyNumbers {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    let value = match *self {
      Self {
        previous_value: None,
        sequence,
        ..
      } => sequence,

      Self {
        previous_value: Some(114),
        ..
      } => 140,

      Self {
        previous_value: Some(previous_value),
        ..
      } if previous_value >= 140 && previous_value < 149 => previous_value + 1,

      Self {
        previous_value: Some(149),
        ..
      } => 214,

      Self {
        previous_value: Some(previous_value),
        order_of_magnitude,
        ..
      } => previous_value + (10 as i32).pow(order_of_magnitude),
    };
    self.previous_value = Some(value);
    Some(value)
  }
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
  // If the sequence does not "fit" in the number, there can be no matches
  if sequence.len() > number.len() {
    return 0;
  }

  let mut result = 0;
  let max_possible_start_index = number.len() - sequence.len() + 1;

  for match_start_index in 0..max_possible_start_index {
    for sequence_index in 0..sequence.len() {
      let number_character = number[match_start_index + sequence_index];
      let sequence_character = sequence[sequence_index];
      if number_character != sequence_character {
        break;
      }

      let is_last_character = sequence_index == (sequence.len() - 1);
      if is_last_character {
        result += 1;
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

#[test]
fn test_lucky_numbers() {
  // Arrange
  let end = 1000;
  let sequence = 14;
  let lucky_numbers = LuckyNumbers::new(sequence);

  // Act
  let result: Vec<i32> = lucky_numbers.take(20).collect();

  // Assert
  assert_eq!(
    result,
    [
      14, 114, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 214, 314, 414, 514, 614, 714, 814,
      914
    ]
  )
}
