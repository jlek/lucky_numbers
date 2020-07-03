// TODO Add tests

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
