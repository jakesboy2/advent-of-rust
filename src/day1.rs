use std::fs;

struct Digits {
  first: Option<char>,
  second: Option<char>
}

fn get_digits_from_line(line: &str) -> Digits {
  let mut digits = Digits {
    first: None,
    second: None
  };

  for i in line.chars() {
    if !i.is_digit(10) {
      continue;
    }

    match digits.first {
      Some(_) => digits.second = Some(i),
      None => digits.first = Some(i)
    }
  }

  return digits;
}

fn get_sum_value(digits: Digits) -> u32 {
  let mut first_digit = digits.first.unwrap().to_digit(10).unwrap();
    match digits.second {
      Some(second_digit) => {
        first_digit *= 10;
        first_digit += second_digit.to_digit(10).unwrap_or(0);
      },
      None => {
        first_digit *= 11;
      }
    }

  return first_digit;
}

fn clean_line_to_digits(line: &mut String) -> &mut String {
  let mut digits_to_insert = Vec::<(String, usize)>::new();
  let number_strings = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
  for (idx, num) in number_strings.iter().enumerate() {
    let results: Vec<_> = line.match_indices(num).map(|(i, _)| (i, idx.to_string())).collect();
    for result in results {
      digits_to_insert.push((String::from(result.1), result.0))
    }
  }

  let mut insertions = 0;
  digits_to_insert.sort_by_key(|k| k.1);
  for digit_tuple in digits_to_insert {
    let from = digit_tuple.0.as_str();
    let index = digit_tuple.1;
    line.insert_str(index + insertions, &from);
    insertions += 1;
  }

  return line;
}

pub fn _part_1() {
  let contents = fs::read_to_string("inputs/d1p1_input.txt")
    .unwrap();

  let mut sum = 0;
  for line in contents.lines() {
    let digits = get_digits_from_line(line);
    let sum_value = get_sum_value(digits);
    sum += sum_value;
  }

  println!("Sum is -> {}", sum);
}

#[allow(dead_code)]
pub fn part_2() {
  let contents = fs::read_to_string("inputs/d1p1_input.txt")
    .unwrap();

  let mut sum = 0;
  for line in contents.lines() {
    let mut curr_line = String::from(line);
    let cleaned_line = clean_line_to_digits(&mut curr_line);
    let digits = get_digits_from_line(cleaned_line.as_str());
    let sum_value = get_sum_value(digits);
    sum += sum_value;
  }

  println!("Sum is -> {}", sum);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_line() {
    let input = &mut String::from("eightwothree");
    let cleaned_line = clean_line_to_digits(input);
    let digits = get_digits_from_line(cleaned_line);
    let value = get_sum_value(digits);
    assert_eq!(value, 83);
  }

  #[test]
  fn test_weird_insertion() {
    let input = &mut String::from("seven5seven6sixmdfrtwo");
    let cleaned_line = clean_line_to_digits(input);
    assert_eq!(cleaned_line, "7seven57seven66sixmdfr2two");
  }
}