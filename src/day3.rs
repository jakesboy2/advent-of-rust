use std::fs;

#[derive(Clone)]
struct Position {
  x: usize,
  y: usize
}

impl Position {
  pub fn new(x: usize, y: usize) -> Self {
    Position { x, y }
  }

  pub fn is_touching_pos(&self, pos: &Position) -> bool {
    // Is it to the right?
    if self.x == pos.x + 1 && self.y == pos.y {
      return true;
    }

    // Is it to the left?
    if pos.x > 0 {
      if self.x == pos.x - 1 && self.y == pos.y {
        return true;
      } 
    }

    // Is it above?
    if pos.y > 0 {
      if self.x == pos.x && self.y == pos.y - 1 {
        return true;
      }
    }

    // Is it below?
    if self.x == pos.x && self.y == pos.y + 1 {
      return true;
    }

    // Is it bottom left?
    if pos.x > 0 && pos.y > 0 {
      if self.x == pos.x - 1 && self.y == pos.y - 1 {
        return true;
      }
    }

    // Is it bottom right?
    if pos.y > 0 {
      if self.x == pos.x + 1 && self.y == pos.y - 1 {
        return true;
      }
    }

    // Is it top left?
    if pos.x > 0 {
      if self.x == pos.x - 1 && self.y == pos.y + 1 {
        return true;
      }
    }

    // Is it top right?
    if self.x == pos.x + 1 && self.y == pos.y + 1 {
      return true;
    }

    return false;
  }
}

#[derive(Clone)]
struct Num {
  val: Vec<char>,
  pos: Vec<Position>,
  is_dirty: bool
}

impl Num {
  pub fn to_i32(&self) -> i32 {
    let str_val: String = self.val.iter().collect::<String>();
    let num_val: i32 = str_val.parse().unwrap();
    num_val
  }
}

impl Default for Num {
  fn default() -> Self {
    Num {
      val: Vec::<char>::new(),
      pos: Vec::<Position>::new(),
      is_dirty: false
    }
  }
}

fn get_matrix(contents: String) -> Vec<Vec<char>> {
  let mut matrix = Vec::<Vec<char>>::new();
  for line in contents.lines() {
    let line_vec: Vec<char> = line.chars().collect();
    matrix.push(line_vec);
  }

  return matrix;
}

fn is_symbol(char_to_test: &char) -> bool {
  let symbols = ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '+', '-', '=', '/'];
  symbols.contains(char_to_test)
}

fn touches_symbol(num: &Num, matrix: &Vec<Vec<char>>) -> bool {
  for pos in num.pos.clone() {
    if let Some(row_below) = matrix.get(pos.y + 1) {
      // Below
      if let Some(bottom_col) = row_below.get(pos.x) {
        if is_symbol(bottom_col) {
          return true;
        }
      }

      // Below Left
      if pos.x > 0 {
        if let Some(bottom_left_col) = row_below.get(pos.x - 1) {
          if is_symbol(bottom_left_col) {
            return true;
          }
        }
      }

      // Below right
      if let Some(bottom_right_col) = row_below.get(pos.x + 1) {
        if is_symbol(bottom_right_col) {
          return true;
        }
      }
    }

    if pos.y > 0 {
      if let Some(row_above) = matrix.get(pos.y - 1) {
        // Above
        if let Some(above_col) = row_above.get(pos.x) {
          if is_symbol(above_col) {
            return true;
          }
        }
  
        // Above Left
        if pos.x > 0 {
          if let Some(above_left_col) = row_above.get(pos.x - 1) {
            if is_symbol(above_left_col) {
              return true;
            }
          }
        }
  
        // Above right
        if let Some(above_right_col) = row_above.get(pos.x + 1) {
          if is_symbol(above_right_col) {
            return true;
          }
        }
      }
    }
  
    if let Some(curr_row) = matrix.get(pos.y) {
      // Left
      if pos.x > 0 {
        if let Some(left_col) = curr_row.get(pos.x - 1) {
          if is_symbol(left_col) {
            return true;
          }
        }
      }

      // Right
      if let Some(right_col) = curr_row.get(pos.x + 1) {
        if is_symbol(right_col) {
          return true;
        }
      }
    }
  }
  
  return false;
}

fn get_sum_from_nums(nums: Vec<Num>, matrix: &Vec<Vec<char>>) -> i32 {
  let mut sum = 0;
  for num in nums {
    if touches_symbol(&num, &matrix) == true {
      sum += num.to_i32();
    }
  }
  sum
}

fn get_nums_from_matrix(matrix: &Vec<Vec<char>>) -> Vec<Num> {
  let mut nums = Vec::<Num>::new();
  for (row_idx, row) in matrix.iter().enumerate() {
    let mut num = Num::default();
    for (col_idx, col) in row.iter().enumerate() {
      if !col.is_digit(10) {
        if num.is_dirty == true {
          nums.push(num.clone());
        }
        num = Num::default();
      }
      else {
        num.val.push(*col);
        let pos = Position::new(col_idx, row_idx);
        num.pos.push(pos);
        num.is_dirty = true;

        if col_idx == row.len() - 1 {
          nums.push(num.clone())
        }
      } 
    }
  }

  return nums;
}

fn get_part_positions(matrix: &Vec<Vec<char>>) -> Vec<Position> {
  let mut parts = Vec::<Position>::new();
  for (row_idx, row) in matrix.iter().enumerate() {
    for (col_idx, col) in row.iter().enumerate() {
      if is_symbol(col) {
        parts.push(Position::new(col_idx, row_idx));
      }
    }
  }

  return parts;
}

fn get_touching_nums<'a>(part: &Position, nums: &'a Vec<Num>) -> Vec<&'a Num> {
  let mut touching_nums = Vec::<&Num>::new();
  for num in nums {
    for char_pos in num.pos.clone() {
      if part.is_touching_pos(&char_pos) {
        touching_nums.push(num);
        break;
      }
    }
  }
  touching_nums
}

fn get_gear_ratio_sum_from_nums(parts: &Vec<Position>, nums: &Vec<Num>) -> i32 {
  let mut sum = 0;
  for part in parts {
    let touching_nums = get_touching_nums(part, nums);
    if touching_nums.len() == 2 {
      let first_part = touching_nums.get(0).unwrap().to_i32();
      let second_part = touching_nums.get(1).unwrap().to_i32();
      sum += first_part * second_part;
    }
  }
  sum
}

pub fn part_1() {
  let contents = fs::read_to_string("inputs/d3_input.txt")
    .unwrap();

  let matrix = get_matrix(contents);
  let nums = get_nums_from_matrix(&matrix);
  let sum = get_sum_from_nums(nums, &matrix);

  println!("Sum is -> {}", sum);
}

pub fn part_2() {
  let contents = fs::read_to_string("inputs/d3_input.txt")
    .unwrap();

  let matrix = get_matrix(contents);
  let nums = get_nums_from_matrix(&matrix);
  let parts = get_part_positions(&matrix);
  let sum = get_gear_ratio_sum_from_nums(&parts, &nums);

  println!("Sum is -> {}", sum);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_nums_from_matrix() {
    let lines = "48....*...3.\n....32....21\n....38..987.";
    let matrix = get_matrix(lines.to_string());
    let nums = get_nums_from_matrix(&matrix);
    assert_eq!(nums.len(), 6);
  }

  #[test]
  fn test_get_sum_from_nums() {
    let lines = 
      ".....\n\
      .11..\n\
      @..22\n\
      33333\n\
      .44.5";

    let matrix = get_matrix(lines.to_string());
    let nums = get_nums_from_matrix(&matrix);
    let val1 = nums.get(0).unwrap();
    let val2 = nums.get(1).unwrap();
    let val3 = nums.get(2).unwrap();
    let val4 = nums.get(3).unwrap();
    let val5 = nums.get(4).unwrap();

    assert_eq!(val1.to_i32(), 11);
    assert_eq!(val2.to_i32(), 22);
    assert_eq!(val3.to_i32(), 33333);
    assert_eq!(val4.to_i32(), 44);
    assert_eq!(val5.to_i32(), 5);
    
    let sum = get_sum_from_nums(nums, &matrix);
    assert_eq!(sum, 33344);
  }
}