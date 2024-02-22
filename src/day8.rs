use std::{fs, time::Instant, collections::HashMap};

#[derive(Debug)]
enum Direction {
  Left,
  Right
}

#[derive(Debug)]
struct PathValue {
  left: String,
  right: String,
}

fn create_directions(contents: &str) -> Vec<Direction> {
  let mut directions = Vec::<Direction>::new();
  let directions_str = contents.lines().next().unwrap();

  for dir in directions_str.chars() {
    match dir {
      'L' => directions.push(Direction::Left),
      'R' => directions.push(Direction::Right),
      _ => ()
    }
  }
  directions
}

fn create_map(contents: &str) -> HashMap<String, PathValue> {
  let mut map = HashMap::<String, PathValue>::new();

  for line in contents.lines().skip(2) {
    let key = line.split('=').next().unwrap().trim();
    let values = line.split('=').skip(1).next().unwrap().trim();
    let mut left_chars = values.split(',').next().unwrap().chars();
    let mut right_chars = values.split(',').skip(1).next().unwrap().chars();
    left_chars.next();
    right_chars.next_back();

    let pv = PathValue {
      left: String::from(left_chars.as_str().trim()),
      right: String::from(right_chars.as_str().trim()),
    };
    map.insert(String::from(key), pv);
  }
  map
}

fn get_keys_ending_in_a(map: &HashMap<String, PathValue>) -> Vec<&str> {
  let mut keys_ending_in_a = Vec::<&str>::new();
  for (key, _) in map {
    if key.ends_with("A") {
      keys_ending_in_a.push(key);
    }
  }
  keys_ending_in_a
}

#[allow(dead_code)]
pub fn part_1() {
  let before = Instant::now();
  let contents = fs::read_to_string("inputs/d8_input.txt")
    .unwrap();
  let directions = create_directions(&contents);
  let map = create_map(&contents);
  
  
  let mut sum = 0;
  let directions_len = directions.len();
  let mut curr = map.get("AAA").unwrap();
  loop {
    let curr_dir = directions.get(sum % directions_len).unwrap();
    let new_key: String;
    match curr_dir {
      Direction::Left => new_key = curr.left.clone(),
      Direction::Right => new_key = curr.right.clone()
    };
    sum += 1;

    if new_key == "ZZZ" {
      break;
    }

    curr = map.get(&String::from(new_key)).unwrap();
  }

  println!("Sum is -> {}", sum);
  println!("Executed in {:?}", before.elapsed());
}

#[allow(unused_assignments)]
pub fn part_2() {
  let before = Instant::now();
  let contents = fs::read_to_string("inputs/d8_input.txt")
  .unwrap();
  let directions = create_directions(&contents);
  let map = create_map(&contents);

  // infinite looping?
  // find out why!

  let mut sum = 0;
  let directions_len = directions.len();
  let mut curr_keys = get_keys_ending_in_a(&map);
  println!("{:?}", curr_keys);
  let mut distances = Vec::<i32>::new();
  for key in curr_keys {
    let mut sum = 0;
    let mut curr = map.get(key).unwrap();
    loop {
      let curr_dir = directions.get(sum % directions_len).unwrap();
      let new_key: String;
      match curr_dir {
        Direction::Left => new_key = curr.left.clone(),
        Direction::Right => new_key = curr.right.clone()
      };
      sum += 1;

      if new_key.ends_with("Z") {
        distances.push(sum as i32);
        break;
      }
  
      curr = map.get(&String::from(new_key)).unwrap();
    }
  }

  // Go find LCM online and that's your answer lol
  println!("{:?}", distances);
  println!("Executed in {:?}", before.elapsed());
}