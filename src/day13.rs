
pub fn earliest_bus(start_time: i32, bus_ids: &Vec<i32>) -> i32 {
  let mut earliest_id = 0;
  let mut earliest_time = 50000;
  for &id in bus_ids {
    let time = id - (start_time % id);
    if time < earliest_time {
      earliest_time = time;
      earliest_id = id;
    }
  }
  return earliest_id * earliest_time;
}

  // Solve using Chinese Remainder Theory 
  // Given x | a (mod n) ---> all values of x such that x % n = a or rather (x - a) % n = 0
  
  // A sequcnce can be derived such that
  // x | a1 (mod n1)
  // x | a2 (mod n2)
  // x2 = a1 * m2 * n2 + a2 * m1 * n1
  //    where m1 and m2 are the Bézout coefficients of n1 and n2
  //    and n1 and n2 are coprime
  // This can be reformulated as
  //    x | x2 (mod n1*n2)
pub fn find_first_contiguous_time(bus_ids_with_offsets: &Vec<(i64, i64)>) -> i128 {
  let first = bus_ids_with_offsets[0];
  let mut n0 = first.0 as i128; // bus id
  let mut a0 = -1 * first.1 as i128; // offset seconds from final timestamp
  for i in 1..bus_ids_with_offsets.len() {
    let ni = bus_ids_with_offsets[i].0 as i128;
    let ai = -1 * bus_ids_with_offsets[i].1 as i128;
    let (m0, mi) = bezout_coefficients(n0, ni);
    let xi = a0 * ni * mi + ai * n0 * m0;
    n0 = n0 * ni;
    a0 = xi;

    // of the many valid values of x, use the one closest to 0
    if a0.abs() > n0 {
      let over = a0 / n0;
      a0 = a0 - over * n0;
    }
  }
  if a0 < 0 {
    return a0 + n0;
  }
  return a0;
}

pub fn read_input() -> String {
  "1007268
  17,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,937,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,23,x,x,x,x,x,29,x,397,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19".to_string()
}

pub fn parse_input_start_time(input: &str) -> (i32, Vec<i32>) {
  let lines = input.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
  let start_time = lines[0].parse().unwrap();
  let bus_ids = lines[1].split(",")
    .filter(|&bus| bus != "x")
    .map(|id| id.parse().unwrap())
    .collect();
  return (start_time, bus_ids);
}

pub fn parse_input_with_offsets(input: &str) -> Vec<(i64, i64)> {
  let mut offset = 0;
  let mut ids_with_offsets = Vec::new();
  let lines = input.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
  for value in lines[1].split(",") {
    if value != "x" {
      ids_with_offsets.push((value.parse().unwrap(), offset));
    }
    offset += 1;
  }
  return ids_with_offsets;
}

// see: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn bezout_coefficients(a: i128, b: i128) -> (i128, i128) {
  let mut s = 0;
  let mut old_s = 1;
  let mut r = b;
  let mut old_r = a;
      
  while r != 0 {
    let quotient = old_r / r;
    let new_r = old_r - quotient * r;
    let new_s = old_s - quotient * s;
    old_r = r;
    r = new_r;
    old_s = s;
    s = new_s;
  }

  let bezout_t;
  if b != 0 {
    bezout_t = (old_r - old_s * a) / b;
  } else {
    bezout_t = 0;
  }
  // Note: Greatest Common Divisor = old_r
  return (old_s, bezout_t);
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_earliest_bus() {
    let input = "939
      7,13,x,x,59,x,31,19";
    let parse = parse_input_start_time(input);
    assert_eq!(295, earliest_bus(parse.0, &parse.1));
  }

  #[test]
  fn test_first_contiguous_timestamp() {
    let input = "939
      7,13,x,x,59,x,31,19";
    let parse = parse_input_with_offsets(&input);
    assert_eq!(1068781, find_first_contiguous_time(&parse));
  }
  
  #[test]
  fn test_gcd() {
    assert_eq!((-9,2), bezout_coefficients(15, 69));
    assert_eq!((2,-1), bezout_coefficients(7, 13));
    assert_eq!((1,19), bezout_coefficients(77, -4));
    assert_eq!((1,-3), bezout_coefficients(-14, -4));
    assert_eq!((-3,1), bezout_coefficients(-4, -14));
    assert_eq!((24,-37), bezout_coefficients(91, 59));
  }
}