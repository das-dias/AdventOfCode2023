use std::fs::read_to_string;
use std::collections::HashMap;

pub fn get_calibration_digits(filename: &str, debug: bool) -> u64 {
  let mut result = 0;
  read_to_string(filename) 
    .unwrap()
    .lines()
    .for_each(|line| {
      result += get_line_digits(line);
      if debug {
        println!("{} : {}", line, get_line_digits(line));
      }
    });
  result
}

fn get_line_digits(line: &str) -> u64 {
  let mut pos_digit_map: HashMap<usize, u64> = HashMap::new(); /* map the found digits to their order */
  let keys = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
  let radix = 10;
  let digits: Vec<(usize, char)> = line.chars()
    .into_iter()
    .enumerate()
    .filter(|(_, c)| c.is_numeric())
    .collect();
  digits
    .iter()
    .for_each(|(idx, c)| {pos_digit_map.insert(idx.clone(), u64::from(c.to_digit(radix).unwrap()));} );
  for (digit, key) in keys.iter().enumerate() {
    let indexes: Vec<usize> = line.match_indices(key).map(|(i, _)|i).collect();
    indexes
      .iter()
      .for_each(|idx| {pos_digit_map.insert(idx.clone(), (digit + 1) as u64);} );
  }
  let map_idxs: Vec<usize> = pos_digit_map.clone().into_keys().collect();
  let first_digit_idx = map_idxs.iter().fold(usize::MAX, |a,b| a.min(*b));
  let last_digit_idx = map_idxs.iter().fold(usize::MIN, |a,b| a.max(*b));
  let last_digit = pos_digit_map.get(&last_digit_idx).unwrap().clone();
  let first_digit = pos_digit_map.get(&first_digit_idx).unwrap().clone();
  let mut result = String::new();
  result.push_str(first_digit.to_string().as_str());
  result.push_str(last_digit.to_string().as_str());
  result.parse().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_get_line_digits() {
    let result = get_line_digits("five74");
    assert_eq!(result, 54);
    let result = get_line_digits("fsid4jf");
    assert_eq!(result, 44);
    let result = get_line_digits("fivesid4jf");
    assert_eq!(result, 54);
    let result = get_line_digits("fisfourjf");
    assert_eq!(result, 44);
    let result = get_line_digits("7pqrstsixteen");
    assert_eq!(result, 76);
    let result = get_line_digits("6zfv66onetwosixtwoxdx");
    assert_eq!(result, 62);
  }
}