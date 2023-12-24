mod day1;
mod day2;

use day1::get_calibration_digits;
use day2::{get_possible_games_id_sum, get_fewest_cubes_power_sum, Config};
use day3::{get_part_number_sum};

fn main() {
  println!("Advent of Code 2023!");
  println!(
    "Day 1 Answer: {}", 
    get_calibration_digits("resources/adventofcode.com_2023_day_1_input.txt", false)
  );
  let day2_config = Config {max_red: 12, max_green: 13, max_blue: 14 };
  println!(
    "Day 2 Answer: {}", 
    get_possible_games_id_sum("resources/adventofcode.com_2023_day_2_input.txt", day2_config)
  );
  println!(
    "Day 2 Part 2 Answer: {}", 
    get_fewest_cubes_power_sum("resources/adventofcode.com_2023_day_2_input.txt")
  );
  println!(
    "Day 3 Part 1 Answer: {}", 
    get_part_number_sum("resources/adventofcode.com_2023_day_3_input.txt")
  )
}

mod day3 {
  use std::fs::read_to_string;
  use std::collections::HashMap;

  const SYMBOLS: [char; 14] = ['!','\"','#','$','%','&','/','(',')','=','?','*','+', '@'];
  type Symbol = usize; /* column */
  type PartNumber = (usize, i64); /* number of digits, number */

  fn parse_symbols(line: & str) -> Vec<Symbol> {
    line.match_indices(&SYMBOLS)
      .into_iter()
      .map(|(col, _)| col)
      .collect()
  }

  fn parse_numbers(line: &str) -> HashMap<usize, PartNumber> { 
    let mut result: HashMap<usize, PartNumber> = HashMap::new();
    let numbers = String::from(line).replace(&SYMBOLS, ".");
    let numbers = numbers.split(".");
    for number_str in numbers {
      if (number_str.len() > 0) && (number_str != "-") {
        let mut digits: Vec<char> = number_str.chars().collect();
        if digits[digits.len()-1] == '-' {
          digits.remove(digits.len()-1);
        }
       let number = String::from_iter(digits);
        result.insert(
          line.find(&number).unwrap(),
          (number.len(), number.parse().unwrap())
        );
      }
    }
    result
  }

  pub fn get_part_number_sum(filename: &str) -> i64 {
    let content = read_to_string(filename)
      .unwrap();
    let lines = content.lines()
      .collect();
    find_part_number_sum(lines)
  }

  fn find_part_number_sum(lines: Vec<& str>) -> i64 { 
    let mut result = 0;
    let mut numbers: HashMap<usize, HashMap<usize, PartNumber>> = HashMap::new();
    let mut symbols: Vec<(usize, Symbol)> = Vec::new();
    lines
      .iter()
      .enumerate()
      .for_each(|(lid, line)| {
        numbers.insert(lid, parse_numbers(line));
        for symbol in parse_symbols(line) {
          symbols.push((lid, symbol));
        }
      });
    let mut possible_symbol_positions: Vec<(usize, usize)>;
    let mut valid_part_numbers: Vec<(usize,usize, i64)> = Vec::new(); /* line, col, number */
    for (lid, part_numbers) in numbers {
      for (col, part_number) in part_numbers {
        /* slow algorithm: check first column and first row inside nested loop */
        for digit_idx in 0..(part_number.0) {
          let cid = col + digit_idx;
          if (lid > 0) && (cid > 0) {
            possible_symbol_positions = vec![
              (lid-1, cid-1), (lid-1, cid), (lid-1, cid+1),
              (lid, cid-1),   /* number */  (lid, cid+1),
              (lid+1, cid-1), (lid+1, cid), (lid+1, cid+1),
            ];
          } else if lid > 0 {
            possible_symbol_positions = vec![
              (lid-1, cid), (lid-1, cid+1),
              /* number */  (lid, cid+1),
              (lid+1, cid), (lid+1, cid+1),
            ];
          } else if cid > 0 {
            possible_symbol_positions = vec![
              (lid, cid-1),   /* number */  (lid, cid+1),
              (lid+1, cid-1), (lid+1, cid), (lid+1, cid+1),
            ];
          } else {
            possible_symbol_positions = vec![
              /* number */  (lid, cid+1),
              (lid+1, cid), (lid+1, cid+1),
            ];
          }
          if possible_symbol_positions.iter().any(|pos| symbols.contains(&pos)) {
            if !valid_part_numbers.contains(&(lid, col, part_number.1)) {
              valid_part_numbers.push((lid, col, part_number.1))
            }
          }
        }
      }
    }
    valid_part_numbers
      .iter()
      .for_each(|(_, _, num)| result += num);
    result
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    #[test]
    fn test_parse_symbols() {
      let symbols = parse_symbols("467..114..");
      assert_eq!( symbols, vec![] );
      let symbols = parse_symbols("467*.114..");
      assert_eq!( symbols, vec![3] );
      let symbols = parse_symbols("...$.*....");
      assert_eq!( symbols, vec![3, 5] );
    }

    #[test]
    fn test_parse_numbers() {
      let numbers = parse_numbers("467..114..");
      assert_eq!( numbers, HashMap::from([(0, (3, 467)), (5, (3, 114))]) );
      let numbers = parse_numbers("467*.114..");
      assert_eq!( numbers, HashMap::from([(0, (3, 467)), (5, (3, 114))]) );
      let numbers = parse_numbers("...$.*....");
      assert_eq!( numbers, HashMap::new() );
    }

    #[test]
    fn test_find_part_number_sum() {
      let sum = find_part_number_sum(vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598.."
      ]);
      assert_eq!(sum, 4361)
    }
  }
}