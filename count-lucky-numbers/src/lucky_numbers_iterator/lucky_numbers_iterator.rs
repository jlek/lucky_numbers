use super::postfixed_lucky_numbers::PostfixedLuckyNumbers;
use super::prefixed_lucky_numbers::PrefixedLuckyNumbers;

/// Iterates over all numbers that contain a lucky sequence at least once.
/// For example, if the sequence is 123, it iterate over 123, 1123, 1230, ...
pub struct LuckyNumbersIterator {
  sequence: u32,
  postfixed_lucky_numbers: PostfixedLuckyNumbers,
  prefixed_lucky_numbers: PrefixedLuckyNumbers,
  next_postfixed_value: u32,
  next_prefixed_value: u32,
  first_value: bool,
}

impl LuckyNumbersIterator {
  pub fn new(sequence: u32) -> Self {
    let mut postfixed_lucky_numbers = PostfixedLuckyNumbers::new(sequence);
    let mut prefixed_lucky_numbers = PrefixedLuckyNumbers::new(sequence);
    let next_postfixed_value = postfixed_lucky_numbers
      .next()
      .expect("There should be at least one value in the lucky number sequence");
    let next_prefixed_value = prefixed_lucky_numbers
      .next()
      .expect("There should be at least one value in the lucky number sequence");

    LuckyNumbersIterator {
      sequence,
      postfixed_lucky_numbers,
      prefixed_lucky_numbers,
      next_postfixed_value,
      next_prefixed_value,
      first_value: true,
    }
  }
}

impl Iterator for LuckyNumbersIterator {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.first_value {
      self.first_value = false;
      return Some(self.sequence);
    }

    if self.next_postfixed_value < self.next_prefixed_value {
      let value = self.next_postfixed_value;
      self.next_postfixed_value = self.postfixed_lucky_numbers.next().expect(
        "the sequence of lucky numbers is infinite, so there should always be a next number",
      );
      Some(value)
    } else {
      let value = self.next_prefixed_value;
      self.next_prefixed_value = self.prefixed_lucky_numbers.next().expect(
        "the sequence of lucky numbers is infinite, so there should always be a next number",
      );
      Some(value)
    }
  }
}

#[cfg(test)]
mod test {
  use super::LuckyNumbersIterator;

  #[test]
  fn double_digit_sequence() {
    // Arrange
    let lucky_numbers = LuckyNumbersIterator::new(14);
    // Act
    let result: Vec<u32> = lucky_numbers.take(20).collect();
    // Assert
    assert_eq!(
      result,
      [
        14, 114, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 214, 314, 414, 514, 614, 714,
        814, 914
      ]
    )
  }
}
