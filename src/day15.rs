use std::collections::HashMap;

pub fn find_nth_number_memo(starting_numbers: &Vec<i32>, nth: usize) -> i32 {
  let mut map: HashMap<i32, usize> = HashMap::new();
  for i in 0..starting_numbers.len() - 1 {
    map.insert(starting_numbers[i], i+1);
  }
  let mut last = *starting_numbers.last().unwrap();
  for i in starting_numbers.len()..nth {
    let next = match map.get(&last) {
      Some(last_pos) => (i - last_pos) as i32,
      None => 0 
    };
    map.insert(last, i);
    last = next;
  }
  return last;
}

pub fn parse_input(input: &str) -> Vec<i32> {
  input.split(",").map(|s| s.parse().unwrap()).collect()
}

pub fn read_input() -> String {
  return "8,0,17,4,1,12".to_string();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_2020th_num() {
    assert_eq!(10, find_nth_number_memo(&vec![2,1,3], 2020));
    assert_eq!(1836, find_nth_number_memo(&vec![3,1,2], 2020));
  }

  #[test]
  fn find_n() {
    assert_eq!(0, find_nth_number_memo(&vec![0,3,6], 10));
  }

  #[test]
  #[ignore]
  fn test_30000000th_num() {
    assert_eq!(362, find_nth_number_memo(&vec![3,1,2], 30000000));
  }
}