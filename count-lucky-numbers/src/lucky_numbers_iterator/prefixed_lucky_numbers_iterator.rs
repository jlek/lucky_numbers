use std::convert::TryInto;

const TEN: u32 = 10;

/// Iterates over all numbers that can be formed by prefixing the sequence with more digits.
/// For example, if the sequence is 123, it would return 1123, 2123, 3123, ...
pub struct PrefixedLuckyNumbersIterator {
  sequence: u32,
  multiplier: u32,
  sequence_length: u32,
}

impl PrefixedLuckyNumbersIterator {
  pub fn new(sequence: u32) -> Self {
    PrefixedLuckyNumbersIterator {
      sequence,
      multiplier: 0,
      sequence_length: sequence
        .to_string()
        .len()
        .try_into()
        .expect("sequence length should fit into a u32"),
    }
  }
}

impl Iterator for PrefixedLuckyNumbersIterator {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.multiplier += 1;
    Some(self.multiplier * TEN.pow(self.sequence_length) + self.sequence)
  }
}

#[cfg(test)]
mod test {
  use super::PrefixedLuckyNumbersIterator;

  #[test]
  fn single_digit_sequence() {
    // Arrange
    let lucky_numbers = PrefixedLuckyNumbersIterator::new(3);

    // Act
    let result: Vec<u32> = lucky_numbers.take(10).collect();

    // Assert
    assert_eq!(result, [13, 23, 33, 43, 53, 63, 73, 83, 93, 103]);
  }

  #[test]
  fn double_digit_sequence() {
    // Arrange
    let lucky_numbers = PrefixedLuckyNumbersIterator::new(14);

    // Act
    let result: Vec<u32> = lucky_numbers.take(10).collect();

    // Assert
    assert_eq!(result, [114, 214, 314, 414, 514, 614, 714, 814, 914, 1014])
  }

  #[test]
  fn triple_digit_sequence() {
    // Arrange
    let lucky_numbers = PrefixedLuckyNumbersIterator::new(987);

    // Act
    let result: Vec<u32> = lucky_numbers.take(10).collect();

    // Assert
    assert_eq!(
      result,
      [1987, 2987, 3987, 4987, 5987, 6987, 7987, 8987, 9987, 10987]
    );
  }
}
