use std::time::Instant;
use std::fs;

#[derive(Debug)]
struct RaceResults {
  time: i64,
  distance: i64
}

fn get_race_results(contents: &str) -> Vec<RaceResults> {
  let mut time_values = Vec::<i64>::new();
  let time_values_str: Vec<&str> = contents.lines().next().unwrap()
    .split(':')
    .skip(1)
    .next().unwrap().trim()
    .split_whitespace().collect();
  for time_val in time_values_str {
    let casted_time = time_val.parse().unwrap();
    time_values.push(casted_time);
  }

  let mut distance_values = Vec::<i64>::new();
  let distance_values_str: Vec<&str> = contents.lines().skip(1).next().unwrap()
    .split(':')
    .skip(1)
    .next().unwrap().trim()
    .split_whitespace().collect();
  for distance_val in distance_values_str {
    let casted_distance = distance_val.parse().unwrap();
    distance_values.push(casted_distance);
  }

  let mut race_results = Vec::<RaceResults>::new();
  for i in 0..distance_values.len() {
    race_results.push(
      RaceResults {
        time: *time_values.get(i).unwrap(),
        distance: *distance_values.get(i).unwrap()
      }
    )
  }
  race_results
}

fn get_race_results_part_2(contents: &str) -> RaceResults {
  let time_values_str: Vec<&str> = contents.lines().next().unwrap()
    .split(':')
    .skip(1)
    .next().unwrap().trim()
    .split_whitespace().collect();
  let time_val: i64 = time_values_str.concat().parse().unwrap();

  let distance_values_str: Vec<&str> = contents.lines().skip(1).next().unwrap()
    .split(':')
    .skip(1)
    .next().unwrap().trim()
    .split_whitespace().collect();
  let distance_val: i64 = distance_values_str.concat().parse().unwrap();

  RaceResults { time: time_val, distance: distance_val }
}

fn get_distance_travelled(button_held_ms: &i64, race_length: &i64) -> i64 {
  let time_to_travel = race_length - button_held_ms;
  let distance_travelled = time_to_travel * button_held_ms;
  distance_travelled
}

fn get_num_ways_to_win(race_result: &RaceResults) -> i64 {
  let mut num_wins = 0;
  for i in 0..=race_result.time {
    let distance_travelled = get_distance_travelled(&i, &race_result.time);
    if distance_travelled > race_result.distance {
      num_wins += 1;
    }
  }
  num_wins
}

#[allow(dead_code)]
pub fn part_1() {
  let before = Instant::now();
  let contents = fs::read_to_string("inputs/d6_input.txt")
    .unwrap();
  let race_results = get_race_results(&contents);
  
  let mut result: Option<i64> = None;
  for race_result in race_results {
    let num_wins = get_num_ways_to_win(&race_result);
    println!("Num wins -> {}", num_wins);
    if result.is_none() {
      result = Some(num_wins);
    }
    else {
      result = Some(result.unwrap() * num_wins);
    }
  }

  println!("Program execution took {:?}", before.elapsed());
  println!("Final Result -> {}", result.unwrap());
}

#[allow(dead_code)]
pub fn part_2() {
  let before = Instant::now();
  let contents = fs::read_to_string("inputs/d6_input.txt")
    .unwrap();
  let race_result = get_race_results_part_2(&contents);
  let num_wins = get_num_ways_to_win(&race_result);

  println!("Program execution took {:?}", before.elapsed());
  println!("Final Result -> {}", num_wins);
}