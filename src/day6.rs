use std::fs;

#[derive(Debug)]
struct RaceResults {
  time: i32,
  distance: i32
}

fn get_race_results(contents: &str) -> Vec<RaceResults> {
  let mut time_values = Vec::<i32>::new();
  let time_values_str: Vec<&str> = contents.lines().next().unwrap()
    .split(':')
    .skip(1)
    .next().unwrap().trim()
    .split_whitespace().collect();
  for time_val in time_values_str {
    let casted_time = time_val.parse().unwrap();
    time_values.push(casted_time);
  }

  let mut distance_values = Vec::<i32>::new();
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

pub fn part_1() {
  let contents = fs::read_to_string("inputs/d6_input.txt")
    .unwrap();
  let race_results = get_race_results(&contents);
  println!("Race results -> {:?}", race_results);
}