use std::{str::FromStr, string::ParseError, fs};

fn get_pull_tuple(pull: &str) -> (i32, &str) {
  let values = pull.trim()
    .split(' ')
    .collect::<Vec<&str>>();

  let number = values.get(0).unwrap().parse::<i32>().unwrap();
  let color = values.get(1).unwrap();

  return (number, color);
}

#[derive(Debug)]
struct Game {
  id: i32,
  red: Vec<i32>,
  green: Vec<i32>,
  blue: Vec<i32>,
}

impl Default for Game {
  fn default() -> Self {
    Game {
      id: 1,
      red: Vec::<i32>::new(),
      green: Vec::<i32>::new(),
      blue: Vec::<i32>::new()
    }
  }
}

impl FromStr for Game {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut game_data = Game::default();

    let game_id = s.split(':').next().unwrap()
      .split(' ')
      .skip(1)
      .next().unwrap();
    game_data.id = game_id.parse::<i32>().unwrap();

    let all_games = s.split(':')
      .skip(1)
      .next().unwrap().trim()
      .split(';')
      .collect::<Vec<&str>>();
    
    for game in all_games {
      let pulls = game.split(',')
        .collect::<Vec<&str>>();

      for pull in pulls {
        let pull_tuple = get_pull_tuple(pull);
        match pull_tuple.1 {
          "red" => game_data.red.push(pull_tuple.0),
          "green" => game_data.green.push(pull_tuple.0),
          "blue" => game_data.blue.push(pull_tuple.0),
          _ => ()
        }
      }
    }

    Ok(game_data)
  }
}

pub fn part_1() {
  let red_max = 12;
  let green_max = 13;
  let blue_max = 14;
  let mut sum = 0;
  let contents = fs::read_to_string("inputs/d2_input.txt")
  .unwrap();

  for line in contents.lines() {
    let game_data = Game::from_str(line).unwrap();
    
    if game_data.red.into_iter().any(|pull| pull > red_max) {
      continue;
    }
    if game_data.green.into_iter().any(|pull| pull > green_max) {
      continue;
    }
    if game_data.blue.into_iter().any(|pull| pull > blue_max) {
      continue;
    }

    sum += game_data.id;
  }

  println!("Sum is -> {}", sum);
}

pub fn part_2() {
  let mut sum = 0;
  let contents = fs::read_to_string("inputs/d2_input.txt")
  .unwrap();

  for line in contents.lines() {
    let game_data = Game::from_str(line).unwrap();
    let max_red = game_data.red.iter().max().unwrap();
    let max_green = game_data.green.iter().max().unwrap();
    let max_blue = game_data.blue.iter().max().unwrap();

    let power = max_blue * max_green * max_red;
    sum += power;
  }

  println!("Sum is -> {}", sum);
}