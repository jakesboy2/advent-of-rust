use std::fs;

struct Digits {
  first: Option<char>,
  second: Option<char>
}
fn main() {
  let contents = fs::read_to_string("inputs/d1p1_input.txt")
    .unwrap();

  let mut sum = 0;

  for line in contents.lines() {
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
    
    sum += first_digit;
  }

  println!("Sum is -> {}", sum);
}
