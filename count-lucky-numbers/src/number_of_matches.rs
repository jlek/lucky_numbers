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

#[cfg(test)]
mod test {
  use super::number_of_matches;

  #[test]
  fn number_too_short() {
    // Arrange
    let number = 12.to_string();
    let sequence = 123.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 0);
  }

  #[test]
  fn single_digit_sequence_single_match() {
    // Arrange
    let number = 151.to_string();
    let sequence = 5.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 1);
  }

  #[test]
  fn single_digit_sequence_no_match() {
    // Arrange
    let number = 151.to_string();
    let sequence = 6.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 0);
  }

  #[test]
  fn single_digit_sequence_multiple_matches() {
    // Arrange
    let number = 55515.to_string();
    let sequence = 5.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 4);
  }

  #[test]
  fn double_digits_sequence_single_match() {
    // Arrange
    let number = 151.to_string();
    let sequence = 15.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 1);
  }

  #[test]
  fn double_digits_sequence_no_match() {
    // Arrange
    let number = 151.to_string();
    let sequence = 16.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 0);
  }

  #[test]
  fn double_digits_sequence_multiple_matches() {
    // Arrange
    #[allow(clippy::unreadable_literal)]
    let number = 15151415.to_string();
    let sequence = 15.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 3);
  }

  #[test]
  fn double_digits_sequence_overlapping_matches() {
    // Arrange
    #[allow(clippy::unreadable_literal)]
    let number = 111.to_string();
    let sequence = 11.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 2);
  }

  #[test]
  fn triple_digits_sequence_single_match() {
    // Arrange
    let number = 1234.to_string();
    let sequence = 123.to_string();

    // Act
    let result = number_of_matches(number.as_bytes(), sequence.as_bytes());

    // Assert
    assert_eq!(result, 1);
  }
}
