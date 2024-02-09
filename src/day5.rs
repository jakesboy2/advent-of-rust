use std::fs;

enum SourceMaps {
  SeedToSoil,
  SoilToFert,
  FertToWater,
  WaterToLight,
  LightToTemp,
  TempToHumidity,
  HumidityToLocation
}

#[derive(Debug)]
struct RangeMap {
  source: i64,
  dest: i64,
  range: i64
}

struct SourceMapDefinition {
  maps: Vec<RangeMap>
}

impl SourceMapDefinition {
  pub fn next(&self, val_to_map: &i64) -> i64 {
    let mut mapped_value: Option<i64> = None;
    for map in &self.maps {
      let min_val = map.source;
      let max_val = map.source + &map.range;

      if val_to_map >= &min_val && val_to_map <= &max_val {
        mapped_value = Some(map.dest + (val_to_map - min_val));
      }
    }

    if mapped_value.is_none() {
      mapped_value = Some(*val_to_map);
    }

    mapped_value.unwrap()
  }
}

fn get_seeds_from_line(line: &str) -> Vec<i64> {
  let seeds_str: Vec<&str> = line.split(':')
    .skip(1)
    .next().unwrap().trim()
    .split_whitespace()
    .collect();

  let mut seeds = Vec::<i64>::new();
  for seed_str in seeds_str {
    let casted_val: i64 = seed_str.parse().unwrap();
    seeds.push(casted_val);
  }
  seeds
}

fn get_source_map_search_string(map_type: &SourceMaps) -> &str {
  match map_type {
    SourceMaps::SeedToSoil => "seed-to-soil map:",
    SourceMaps::SoilToFert => "soil-to-fertilizer map:",
    SourceMaps::FertToWater => "fertilizer-to-water map:",
    SourceMaps::WaterToLight => "water-to-light map:",
    SourceMaps::LightToTemp => "light-to-temperature map:",
    SourceMaps::TempToHumidity => "temperature-to-humidity map:",
    SourceMaps::HumidityToLocation => "humidity-to-location map:"
  }
}

fn create_range_map_from_line(range_map_str: &str) -> RangeMap {
  let map: Vec<&str> = range_map_str.split_whitespace().collect();
  let dest: i64 = map.get(0).unwrap().parse().unwrap();
  let source: i64 = map.get(1).unwrap().parse().unwrap();
  let range: i64 = map.get(2).unwrap().parse().unwrap();

  RangeMap { source, dest, range}
}

fn create_source_map(map_type: SourceMaps, contents: &str) -> SourceMapDefinition {
  let search_string = get_source_map_search_string(&map_type);
  let mut range_maps = Vec::<RangeMap>::new();
  let mut found = false;
  for line in contents.lines() {
    if line.contains(search_string) {
      found = true;
      continue;
    }

    if found {
      if line.trim().is_empty() {
        break;
      }

      let range_map = create_range_map_from_line(line);
      range_maps.push(range_map);
      continue;
    }
  }

  return SourceMapDefinition { maps: range_maps };
}

pub fn part_1() {
  let contents = fs::read_to_string("inputs/d5_input.txt")
    .unwrap();
  
  let seed_line = contents.lines().next().unwrap();
  let seeds = get_seeds_from_line(seed_line);
  let seed_to_soil = create_source_map(SourceMaps::SeedToSoil, &contents);
  let soil_to_fert = create_source_map(SourceMaps::SoilToFert, &contents);
  let fert_to_water = create_source_map(SourceMaps::FertToWater, &contents);
  let water_to_light = create_source_map(SourceMaps::WaterToLight, &contents);
  let light_to_temp = create_source_map(SourceMaps::LightToTemp, &contents);
  let temp_to_humidity = create_source_map(SourceMaps::TempToHumidity, &contents);
  let humidity_to_location = create_source_map(SourceMaps::HumidityToLocation, &contents);

  let mut lowest_value = 0;
  for seed in seeds {
    let value = humidity_to_location.next(
      &temp_to_humidity.next(
        &light_to_temp.next(
          &water_to_light.next(
            &fert_to_water.next(
              &soil_to_fert.next(
                &seed_to_soil.next(&seed)
              )
            )
          )
        )
      )
    );

    if lowest_value == 0 {
      lowest_value = value;
    }
    else if lowest_value > value {
      lowest_value = value;
    }
  }

  println!("Lowest value -> {}", lowest_value);
}