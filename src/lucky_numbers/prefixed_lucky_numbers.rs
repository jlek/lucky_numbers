pub struct PrefixedLuckyNumbers {
  sequence: i32,
  multiplier: i32,
  sequence_length: usize,
}

impl PrefixedLuckyNumbers {
  pub fn new(sequence: i32) -> Self {
    Self {
      sequence,
      multiplier: 0,
      sequence_length: sequence.to_string().len(),
    }
  }
}

impl Iterator for PrefixedLuckyNumbers {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    self.multiplier += 1;

    Some(self.multiplier * (10 as i32).pow(self.sequence_length as u32) + self.sequence)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn single_digit_sequence() {
    // Arrange
    let lucky_numbers = PrefixedLuckyNumbers::new(3);

    // Act
    let result: Vec<i32> = lucky_numbers.take(10).collect();

    // Assert
    assert_eq!(result, [13, 23, 33, 43, 53, 63, 73, 83, 93, 103]);
  }

  #[test]
  fn double_digit_sequence() {
    // Arrange
    let lucky_numbers = PrefixedLuckyNumbers::new(14);

    // Act
    let result: Vec<i32> = lucky_numbers.take(10).collect();

    // Assert
    assert_eq!(result, [114, 214, 314, 414, 514, 614, 714, 814, 914, 1014])
  }

  #[test]
  fn triple_digit_sequence() {
    // Arrange
    let lucky_numbers = PrefixedLuckyNumbers::new(987);

    // Act
    let result: Vec<i32> = lucky_numbers.take(10).collect();

    // Assert
    assert_eq!(
      result,
      [1987, 2987, 3987, 4987, 5987, 6987, 7987, 8987, 9987, 10987]
    );
  }
}
