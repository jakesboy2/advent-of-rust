use std::{time::Instant, fs, str::FromStr, string::ParseError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum HandResult {
  FiveOfAKind,
  FourOfAKind,
  FullHouse,
  ThreeOfAKind,
  TwoPair,
  OnePair,
  HighCard
}

fn mode(vec_to_mode: &Vec<char>) -> char {
  let mut occurrences = HashMap::new();

  for &value in vec_to_mode {
    if value == 'J' {
      continue;
    }
    *occurrences.entry(value).or_insert(0) += 1;
  }

  if occurrences.len() == 0 {
    return 'J';
  }

  occurrences
    .into_iter()
    .max_by_key(|&(_, count)| count)
    .map(|(val, _)| val)
    .expect("Cannot compute the mode of zero numbers")
}

impl HandResult {
  pub fn check_five_of_a_kind(hand: &Vec<char>) -> bool {
    let mode_char = mode(&hand);
    let mut updated_hand = Vec::<char>::new();
    for i in hand {
      if i == &'J' {
        updated_hand.push(mode_char);
      }
      else {
        updated_hand.push(*i);
      }
    }

    let first_element = updated_hand.get(0).unwrap();
    let result = updated_hand.iter().all(|x| x == first_element);
    result
  }

  pub fn check_four_of_a_kind(hand: &Vec<char>) -> bool {
    let mode_char = mode(&hand);
    let mut updated_hand = Vec::<char>::new();
    for i in hand {
      if i == &'J' {
        updated_hand.push(mode_char);
      }
      else {
        updated_hand.push(*i);
      }
    }
    let mut count;
    for i in 0..updated_hand.len() {
      let comparing_element = updated_hand.get(i).unwrap();
      count = 0;
      for j in 0..updated_hand.len() {
        let curr_element = updated_hand.get(j).unwrap();
        if curr_element == comparing_element {
          count += 1;
        }
      }

      if count == 4 {
        return true
      }
    }
    
    false
  }

  pub fn check_full_house(hand: &Vec<char>) -> bool {
    let mode_char = mode(&hand);
    let mut updated_hand = Vec::<char>::new();
    for i in hand {
      if i == &'J' {
        updated_hand.push(mode_char);
      }
      else {
        updated_hand.push(*i);
      }
    }

    let mut hand_map: HashMap<char, i32>  = HashMap::new();
    for i in &updated_hand {
      if hand_map.contains_key(i) {
        *hand_map.get_mut(i).unwrap() += 1;
      }
      else {
        hand_map.insert(*i, 1);
      }
    }
    
    if hand_map.iter().skip(1).next().is_none() {
      return false;
    }

    let first_value = hand_map.iter().next().unwrap().1;
    let second_value = hand_map.iter().skip(1).next().unwrap().1;
    return (first_value == &2 && second_value == &3) || (first_value == &3 && second_value == &2);
  }

  pub fn check_three_of_a_kind(hand: &Vec<char>) -> bool {
    let mode_char = mode(&hand);
    let mut updated_hand = Vec::<char>::new();
    for i in hand {
      if i == &'J' {
        updated_hand.push(mode_char);
      }
      else {
        updated_hand.push(*i);
      }
    }

    let mut count;
    for i in 0..updated_hand.len() {
      let comparing_element = updated_hand.get(i).unwrap();
      count = 0;
      for j in 0..updated_hand.len() {
        let curr_element = updated_hand.get(j).unwrap();
        if curr_element == comparing_element {
          count += 1;
        }
      }

      if count == 3 {
        return true;
      }
    }
    
    false
  }

  pub fn check_two_pair(hand: &Vec<char>) -> bool {
    if hand.contains(&'J') {
      return false;
    }

    let mut pairs = 0;
    for card in hand {
      let mut num_matches = 0;
      for i in 0..hand.len() {
        let curr_element = hand.get(i).unwrap();
        if curr_element == card {
          num_matches += 1;
        }
      }

      if num_matches == 2 {
        pairs += 1
      }
    }
    
    // we look for 4 because they will match twice when iterating
    pairs == 4
  }

  pub fn check_one_pair(hand: &Vec<char>) -> bool {
    if hand.contains(&'J') {
      return true;
    }

    let mut count;
    for i in 0..hand.len() {
      let comparing_element = hand.get(i).unwrap();
      count = 0;
      for j in 0..hand.len() {
        let curr_element = hand.get(j).unwrap();
        if curr_element == comparing_element {
          count += 1;
        }
      }

      if count == 2 {
        return true;
      }
    }
    
    false
  }

  pub fn get_rank_value(&self) -> i32 {
    match self {
      HandResult::FiveOfAKind => 6,
      HandResult::FourOfAKind => 5,
      HandResult::FullHouse => 4,
      HandResult::ThreeOfAKind => 3,
      HandResult::TwoPair => 2,
      HandResult::OnePair => 1,
      HandResult::HighCard => 0,
    }
  }
}

#[derive(Debug, Clone)]
struct Hand {
  hand: Vec<char>,
  result: HandResult,
  bet: i32
}

impl FromStr for Hand {
  type Err = ParseError;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let results: Vec<&str> = line.split_whitespace().collect();
    let hand: Vec<char> = results.get(0).unwrap().trim().chars().collect();
    let bet = results.get(1).unwrap().trim().parse().unwrap();
    let hand_result = get_hand_result_from_hand(&hand);
    let hand_struct = Hand {
      hand,
      result: hand_result,
      bet
    };

    Ok(hand_struct)
  }
}

fn get_hand_result_from_hand(hand: &Vec<char>) -> HandResult {
  if HandResult::check_five_of_a_kind(hand) {
    return HandResult::FiveOfAKind;
  }
  else if HandResult::check_four_of_a_kind(hand) {
    return HandResult::FourOfAKind;
  }
  else if HandResult::check_full_house(hand) {
    return HandResult::FullHouse;
  }
  else if HandResult::check_three_of_a_kind(hand) {
    return HandResult::ThreeOfAKind;
  }
  else if HandResult::check_two_pair(hand) {
    return HandResult::TwoPair;
  }
  else if HandResult::check_one_pair(hand) {
    return HandResult::OnePair;
  }
  else {
    return HandResult::HighCard;
  }
}

fn get_num_for_face_value(face_value: &char) -> i32 {
  match face_value {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'T' => 11,
    '9' => 10,
    '8' => 9,
    '7' => 8,
    '6' => 7,
    '5' => 6,
    '4' => 5,
    '3' => 4,
    '2' => 3,
    '1' => 2,
    'J' => 1,
    _ => 0
  }
}

fn is_hand_better(hand: &Hand, comparing_hand: &Hand) -> bool {
  let hand_rank = HandResult::get_rank_value(&hand.result);
  let comparing_hand_rank = HandResult::get_rank_value(&comparing_hand.result);

  if hand_rank == comparing_hand_rank {
    for i in 0..hand.hand.len() {
      let hand_card = hand.hand.get(i).unwrap();
      let comparing_card = comparing_hand.hand.get(i).unwrap();
      if hand_card == comparing_card {
        continue;
      }
      return get_num_for_face_value(hand_card) > get_num_for_face_value(comparing_card);
    }
  }

  hand_rank > comparing_hand_rank
}

fn find_highest_ranked_hand_index(hands: &Vec<Hand>) -> usize {
  let mut highest_hand_index = 0;
  for (i, hand) in hands.iter().enumerate() {
    let comparing_hand = hands.get(highest_hand_index).unwrap();
    if is_hand_better(hand, comparing_hand) {
      highest_hand_index = i;
    }
  }
  highest_hand_index
}

fn sort_hands_by_strength(mut hands: Vec<Hand>) -> Vec<Hand> {
  let mut new_hands = Vec::<Hand>::new();
  while hands.len() > 0 {
    let highest_ranked_hand_index = find_highest_ranked_hand_index(&hands);
    new_hands.push(hands.get(highest_ranked_hand_index).unwrap().clone());
    hands.remove(highest_ranked_hand_index);
  }

  new_hands.reverse();
  new_hands
}

#[allow(dead_code)]
fn print_all_for_debug(sorted_hands: &Vec<Hand>) {
  for hand in sorted_hands {
    let hand_string: String = hand.hand.iter().collect();
    let value = &hand.result;
    println!("{} - {:?}", hand_string, value);
  }
}

#[allow(dead_code)]
pub fn part_1() {
  let before = Instant::now();
  let contents = fs::read_to_string("inputs/d7_input.txt")
    .unwrap();

  let mut hands = Vec::<Hand>::new();
  for line in contents.lines() {
    let hand = Hand::from_str(line).unwrap();
    hands.push(hand);
  }
  let sorted_hands = sort_hands_by_strength(hands);
  let mut sum = 0;
  for (i, hand) in sorted_hands.iter().enumerate() {
    let mult = (i as i32) + 1;
    let hand_value = mult * hand.bet;
    sum += hand_value;
  }

  print_all_for_debug(&sorted_hands);

  println!("Sum is -> {}", sum);
  println!("Executed in {:?}", before.elapsed());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compare_hands() {
    let hand1 = Hand {
      hand: vec!['3', '3', '4', '3', '3'],
      bet: 100,
      result: HandResult::FourOfAKind
    };

    let hand2 = Hand {
      hand: vec!['3', '3', '3', '3', '4'],
      bet: 100,
      result: HandResult::FourOfAKind
    };

    let result = is_hand_better(&hand1, &hand2);
    assert_eq!(result, true);
  }

  #[test]
  fn test_full_house() {
    let full_house = vec!['3', '3', '3', '2', '2'];
    let is_full_house = HandResult::check_full_house(&full_house);
    assert_eq!(is_full_house, true);
  }

  #[test]
  fn test_five_of_kind_with_jokers() {
    let five_of_kind_with_jokers = vec!['K', 'K', 'J', 'J', 'K'];
    let is_fok = HandResult::check_five_of_a_kind(&five_of_kind_with_jokers);
    assert_eq!(is_fok, true);
  }

  #[test]
  fn test_four_of_kind_with_jokers() {
    let four_of_kind_with_jokers = vec!['K', 'K', 'J', 'J', '1'];
    let is_fok = HandResult::check_four_of_a_kind(&four_of_kind_with_jokers);
    assert_eq!(is_fok, true);
  }

  #[test]
  fn test_three_of_kind_with_jokers() {
    let three_of_kind_with_jokers = vec!['K', 'J', 'J', '2', '1'];
    let is_tok = HandResult::check_three_of_a_kind(&three_of_kind_with_jokers);
    assert_eq!(is_tok, true);
  }

  #[test]
  fn test_two_pair_with_jokers() {
    let two_pairs_with_jokers = vec!['K', 'J', 'J', '2', '1'];
    let is_two_pair = HandResult::check_two_pair(&two_pairs_with_jokers);
    assert_eq!(is_two_pair, true);
  }

  #[test]
  fn test_one_pair_with_jokers() {
    let one_pair_with_jokers = vec!['K', '1', '2', '3', 'J'];
    let is_one_pair = HandResult::check_one_pair(&one_pair_with_jokers);
    assert_eq!(is_one_pair, true);
  }

  #[test]
  fn test_is_full_house_with_jokers() {
    let full_house_with_jokers = vec!['K', 'K', '2', '2', 'J'];
    let is_full_house = HandResult::check_full_house(&full_house_with_jokers);
    assert_eq!(is_full_house, true);
  }

  #[test]
  fn test_is_five_of_a_kind_with_jokers() {
    let five_of_a_kind_with_jokers = vec!['J', 'J', 'Q', 'Q', 'Q'];
    let is_five_of_a_kind = HandResult::check_five_of_a_kind(&five_of_a_kind_with_jokers);
    assert_eq!(is_five_of_a_kind, true);
  }

  #[test]
  fn test_two_pair_fail() {
    let two_pair_joker = vec!['J', '5', '2', 'A', '9'];
    let is_two_pair = HandResult::check_two_pair(&two_pair_joker);
    assert_eq!(is_two_pair, false);
  }

  #[test]
  fn test_mode() {
    let test = vec!['K', 'K', 'J', '5', '3'];
    let result = mode(&test);
    assert_eq!(result, 'K');
  }
}