mod day1;
use day1::get_calibration_digits;
use day2::{get_possible_games_id_sum, Config};

use crate::day2::get_fewest_cubes_power_sum;

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
  )
}

mod day2 {
  use std::fs::read_to_string;
  use std::collections::HashMap;
  struct Game {
    id: usize,
    draws: Vec<HashMap<String, usize>>
  }

  pub struct Config {
    pub max_red: usize,
    pub max_green: usize,
    pub max_blue: usize,
  }

  fn parse_game(line: &str) -> Game {
    let mut draws = Vec::new();
    let gameid_draw: Vec<& str> = line.split(":").collect();
    let id: usize = gameid_draw[0]
      .split(" ")
      .collect::<Vec<& str>>()[1].parse().unwrap();
    let draw_set = gameid_draw[1].split(";");
    for draw_subset in draw_set {
      let cube_draws = draw_subset.split(",");
      let mut draw = HashMap::new();
      cube_draws.into_iter().for_each(|cd| {
        let cube_draw: Vec<& str> = cd.split(" ").collect();
        let cube_color = String::from(cube_draw[2]);
        let cube_draw_number: usize = cube_draw[1].parse().unwrap();
        draw.insert(cube_color, cube_draw_number);
      });
      draws.push(draw);
    }
    Game {
      id,
      draws
    }
  }

  fn is_valid_game(game: &Game, config: &Config) -> bool {
    for draw in &game.draws {
      let draw_inspections: Vec<bool> = draw.iter().map( 
        |(color, draw_number)| -> bool {
          match color.as_str() {
            "red" => draw_number > &config.max_red,
            "green" => draw_number > &config.max_green,
            "blue" => draw_number > &config.max_blue,
            _ => false
          }
      }).collect();
      if draw_inspections.iter().any(|val| *val) {return false;}
    }
    true
  }

  pub fn get_possible_games_id_sum(filename: & str, config: Config) -> usize {
    let mut result = 0;
    let mut possible_games: Vec<Game> = Vec::new();
    read_to_string(filename) 
      .unwrap()
      .lines()
      .for_each(|line| {
        let game = parse_game(line);
        if is_valid_game(&game, &config){ possible_games.push(game); }
      });
    possible_games.iter().for_each(|g| result += g.id);
    result
  }

  fn get_fewest_cubes_power(game: &Game) -> usize {
    let mut fewest_red: usize = 0;
    let mut fewest_green: usize = 0;
    let mut fewest_blue: usize = 0;
    for draw in &game.draws {
      draw.iter().for_each(|(color, num)| {
        match color.as_str() {
          "red"   => if *num > fewest_red {fewest_red = *num} ,
          "green" => if *num > fewest_green {fewest_green = *num} ,
          "blue"  => if *num > fewest_blue {fewest_blue = *num} ,
          _ => ()
        }
      });
    }
    fewest_red * fewest_green * fewest_blue
  }

  pub fn get_fewest_cubes_power_sum(filename: & str) -> usize {
    let mut result = 0;
    read_to_string(filename) 
      .unwrap()
      .lines()
      .for_each(|line| {
        let game = parse_game(line);
        result += get_fewest_cubes_power(&game);
      });
    result
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    #[test]
    fn test_parse_game() {
      let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
      let game_draw1 = HashMap::from([
        (String::from("blue"), 3),
        (String::from("red"), 4)
      ]);
      assert_eq!(game.id, 1);
      assert_eq!(
        game.draws[0], 
        game_draw1
      );
    }

    #[test]
    fn test_is_valid_game() {
      let game = parse_game("Game 1: 3 blue, 15 red; 1 red, 2 green, 6 blue; 2 green");
      let config1 = Config {
        max_blue: 10,
        max_green: 10,
        max_red: 10
      };
      let config2 = Config {
        max_blue: 10,
        max_green: 10,
        max_red: 20
      };
      assert_eq!(is_valid_game(&game, &config1), false);
      assert_eq!(is_valid_game(&game, &config2), true);
    }

    #[test]
    fn test_get_fewest_cubes_power() {
      let game = parse_game("Game 1: 3 blue, 15 red; 1 red, 2 green, 6 blue; 2 green");
      assert_eq!(get_fewest_cubes_power(&game), 15 * 2 * 6);
    }
  }
}