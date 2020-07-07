const TEN: u32 = 10;

/// Iterates over all numbers that can be formed by postfixing the sequence with more digits.
/// For example, if the sequence is 123, it would return 1230, 1231, 1232, ...
pub struct PostfixedLuckyNumbersIterator {
  sequence: u32,
  previous_postfix: Option<u32>,
  max_postfix: u32,
  order_of_magnitude: u32,
}

impl PostfixedLuckyNumbersIterator {
  pub fn new(sequence: u32) -> Self {
    PostfixedLuckyNumbersIterator {
      sequence,
      previous_postfix: None,
      max_postfix: 10,
      order_of_magnitude: 1,
    }
  }
}

impl Iterator for PostfixedLuckyNumbersIterator {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    let mut postfix = match self.previous_postfix {
      None => 0,
      Some(previous_postfix) => previous_postfix + 1,
    };

    if postfix == self.max_postfix {
      postfix = 0;
      self.max_postfix *= 10;
      self.order_of_magnitude += 1;
    }

    self.previous_postfix = Some(postfix);
    Some(self.sequence * TEN.pow(self.order_of_magnitude) + postfix)
  }
}

#[cfg(test)]
mod test {
  use super::PostfixedLuckyNumbersIterator;

  #[test]
  fn single_digit_sequence() {
    // Arrange
    let lucky_numbers = PostfixedLuckyNumbersIterator::new(3);

    // Act
    let result: Vec<u32> = lucky_numbers.take(20).collect();

    // Assert
    assert_eq!(
      result,
      [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309]
    );
  }

  #[test]
  fn double_digit_sequence() {
    // Arrange
    let lucky_numbers = PostfixedLuckyNumbersIterator::new(14);

    // Act
    let result: Vec<u32> = lucky_numbers.take(20).collect();

    // Assert
    assert_eq!(
      result,
      [
        140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 1400, 1401, 1402, 1403, 1404, 1405, 1406,
        1407, 1408, 1409
      ]
    )
  }

  #[test]
  fn triple_digit_sequence() {
    // Arrange
    let lucky_numbers = PostfixedLuckyNumbersIterator::new(987);

    // Act
    let result: Vec<u32> = lucky_numbers.take(20).collect();

    // Assert
    assert_eq!(
      result,
      [
        9870, 9871, 9872, 9873, 9874, 9875, 9876, 9877, 9878, 9879, 98700, 98701, 98702, 98703,
        98704, 98705, 98706, 98707, 98708, 98709
      ]
    );
  }
}
