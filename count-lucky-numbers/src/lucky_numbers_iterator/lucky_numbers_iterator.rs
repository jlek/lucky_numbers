use super::postfixed_lucky_numbers_iterator::PostfixedLuckyNumbersIterator;
use super::prefixed_lucky_numbers_iterator::PrefixedLuckyNumbersIterator;

/// Iterates over all numbers that contain a lucky sequence at least once.
/// For example, if the sequence is 123, it iterate over 123, 1123, 1230, ...
pub struct LuckyNumbersIterator {
  sequence: u32,
  postfixed_numbers: PostfixedLuckyNumbersIterator,
  prefixed_numbers: PrefixedLuckyNumbersIterator,
  next_postfixed_number: u32,
  next_prefixed_number: u32,
  first_value: bool,
}

impl LuckyNumbersIterator {
  pub fn new(sequence: u32) -> Self {
    let mut postfixed_numbers = PostfixedLuckyNumbersIterator::new(sequence);
    let mut prefixed_numbers = PrefixedLuckyNumbersIterator::new(sequence);
    let next_postfixed_number = postfixed_numbers
      .next()
      .expect("There should be at least one value in the lucky number sequence");
    let next_prefixed_number = prefixed_numbers
      .next()
      .expect("There should be at least one value in the lucky number sequence");

    LuckyNumbersIterator {
      sequence,
      postfixed_numbers,
      prefixed_numbers,
      next_postfixed_number,
      next_prefixed_number,
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

    if self.next_postfixed_number < self.next_prefixed_number {
      let number = self.next_postfixed_number;
      self.next_postfixed_number = self.postfixed_numbers.next().expect(
        "the sequence of lucky numbers is infinite, so there should always be a next number",
      );
      Some(number)
    } else {
      let number = self.next_prefixed_number;
      self.next_prefixed_number = self.prefixed_numbers.next().expect(
        "the sequence of lucky numbers is infinite, so there should always be a next number",
      );
      Some(number)
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
