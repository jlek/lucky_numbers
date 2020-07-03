use super::lucky_numbers_iterator::LuckyNumbersIterator;
use super::number_of_matches::number_of_matches;

#[allow(clippy::needless_pass_by_value)] // Accept the request by value: the request is consumed to produce the response
#[must_use]
pub fn count_lucky_numbers(start: u32, end: u32, sequence: u32) -> usize {
  let sequence_string = sequence.to_string();
  let sequence_bytes = sequence_string.as_bytes();

  LuckyNumbersIterator::new(sequence)
    .skip_while(|number| *number < start)
    .take_while(|number| *number <= end)
    .fold(0, |accumulator, number| {
      accumulator + number_of_matches(number.to_string().as_bytes(), sequence_bytes)
    })
}

#[cfg(test)]
mod test {
  use super::count_lucky_numbers;

  #[test]
  fn double_digit_sequence() {
    // Arrange
    let start = 100;
    let end = 150;
    let sequence = 14;
    // Act
    let result = count_lucky_numbers(start, end, sequence);
    // Assert
    assert_eq!(result, 11); // 114, 140 ... 149
  }
  #[test]
  fn triple_sequence_numbers() {
    // Arrange
    let start = 0;
    let end = 1000;
    let sequence = 123;
    // Act
    let result = count_lucky_numbers(start, end, sequence);
    // Assert
    assert_eq!(result, 1);
  }
}
