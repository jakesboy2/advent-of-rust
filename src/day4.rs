use std::{fs, str::FromStr, string::ParseError,};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Card {
  card_number: i32,
  numbers: Vec<i32>,
  winning_numbers: Vec<i32>,
  wins: i32
}

impl FromStr for Card {
  type Err = ParseError;

  fn from_str (line: &str) -> Result<Self, Self::Err> {
    let game_number: i32 = line.split(':')
      .next().unwrap()
      .split_whitespace()
      .skip(1)
      .next().unwrap().trim()
      .parse().unwrap();

    let game_card = line.split(':')
      .skip(1)
      .next().unwrap().trim();

    let numbers_strings: Vec<&str> = game_card.split('|').
      next().unwrap().trim()
      .split_whitespace().collect();
    let winning_numbers_strings: Vec<&str> = game_card.split('|')
      .skip(1)
      .next().unwrap().trim()
      .split_whitespace().collect();

    let mut numbers = Vec::<i32>::new();
    for num in numbers_strings {
      let casted_val: i32 = num.parse().unwrap();
      numbers.push(casted_val);
    }
    let mut winning_numbers = Vec::<i32>::new();
    for winning_num in winning_numbers_strings {
      let winning_casted_val: i32 = winning_num.parse().unwrap();
      winning_numbers.push(winning_casted_val);
    }

    let wins: i32 = get_number_of_wins(&numbers, &winning_numbers);

    return Ok(Card {
      card_number: game_number,
      numbers,
      winning_numbers,
      wins
    })
  }
}

fn get_number_of_wins(numbers: &Vec<i32>, winning_numbers: &Vec<i32>) -> i32 {
  let mut wins = 0;
  for num in numbers {
    if winning_numbers.contains(num) {
      wins += 1;
    }
  }
  wins
}

fn get_points(matching_numbers: i32) -> i32 {
  if matching_numbers == 0 {
    return 0;
  }

  let base: i32 = 2;
  let exp: u32 = u32::try_from(matching_numbers - 1).unwrap();
  base.pow(exp)
}

pub fn part_1() {
  let contents = fs::read_to_string("inputs/test_input.txt")
  .unwrap();

  let mut sum = 0;
  for line in contents.lines() {
    let card = Card::from_str(line).unwrap();
    let points = get_points(card.wins);
    sum += points;
  }

  println!("Sum is -> {}", sum);
}

pub fn part_2() {
  let contents = fs::read_to_string("inputs/d4_input.txt")
  .unwrap();

let mut card_map: HashMap<i32, Card> = HashMap::new();
let mut total_cards = Vec::<i32>::new();
for line in contents.lines() {
  let card = Card::from_str(line).unwrap();
  card_map.insert(card.card_number, card.clone());
  total_cards.push(card.card_number);
}

  let mut sum = 0;
  while total_cards.len() > 0 {
    let total_cards_clone = total_cards.clone();
    total_cards.clear();
    for card_num in total_cards_clone {
      sum += 1;
      let card_ref = card_map.get(&card_num).unwrap();
      let start_val = card_ref.card_number + 1;
      let end_val = start_val + card_ref.wins;
      for i in start_val..end_val {
        total_cards.push(i);
      }
    }
  }

  println!("Sum is -> {}", sum);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_points() {
    let points = get_points(0);
    assert_eq!(points, 0);
    let points = get_points(2);
    assert_eq!(points, 2);
    let points = get_points(4);
    assert_eq!(points, 8);
    let points = get_points(6);
    assert_eq!(points, 32);
  }

  #[test]
  fn test_card_from_str() {
    let string_val = "Card   1: 77 45  9 81 96  5 91  3 66 |  7 56 66 49 96 58  54 34 37";
    let card = Card::from_str(string_val).unwrap();
    assert_eq!(card.card_number, 1);
    assert_eq!(card.numbers.len(), 9);
    assert_eq!(card.winning_numbers.len(), 9);

    let wins = get_number_of_wins(&card.numbers, &card.winning_numbers);
    assert_eq!(wins, 2);
  }
}