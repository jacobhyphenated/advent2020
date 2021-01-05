
/*
  Day 10: Adapter Array

  Battery adapters can be used for 1, 2, or 3 "jolts" lower than its rating.
  Your device has a build in adapter that's rated for 3 jolts higher than your largest adapter.
  The charging outlet has a rating of 0 jolts.

  Part 1
  Use every adapter in your bag. Count the differences in jolts between each adapter.
  What is the number of 1-jolt differences multiplied be the number of 3-jolt differences?

  Part 2
  What is the total number of distinct ways you can arrange the adapters to connect the carging outlet to your device?
*/

pub fn jolt_diff_using_all_adapters(adapters: &Vec<i64>) -> i64 {
  let mut jolt_diff1 = 0;
  let mut jolt_diff3 = 0;
  let mut current_jolt_val = 0;
  for adapter in adapters {
    let diff = adapter - current_jolt_val;
    if diff < 0 || diff > 3 {
      panic!("Adapter {} could not be applied to current joltage {}", adapter, diff);
    }
    if diff == 1 {
      jolt_diff1 += 1;
    } else if diff == 3 {
      jolt_diff3 += 1
    }
    current_jolt_val = *adapter;
  }
  return jolt_diff1 * jolt_diff3;
}

pub fn total_configurations(adapters: &Vec<i64>) -> i64 {
  // slice array wherever there is a gap of 3
  let mut slice_index_start = 0;
  let mut current_jolt_val = 0;
  let mut slices: Vec<&[i64]> = Vec::new();
  for i in 0..adapters.len() {
    let diff = adapters[i] - current_jolt_val;
    if diff == 3 {
      slices.push(&adapters[slice_index_start..i]);
      slice_index_start = i;
    }
    current_jolt_val = adapters[i];
  }

  // separately compute total combinations for each slice
  // multiply them together
  return slices.iter()
    .map(|slice| get_all_combos(slice))
    .product();
}

fn get_all_combos(adapter_slice: &[i64]) -> i64 {
  if adapter_slice.len() <= 2 {
    return 1;
  }
  let mut j = 1;
  let first = adapter_slice[0];
  let mut combo_list = Vec::new();
  while j < adapter_slice.len() && adapter_slice[j] - first <= 3 {
    combo_list.push(get_all_combos(&adapter_slice[j..]));
    j += 1;
  }
  return combo_list.iter().sum();
}

pub fn read_adapters() -> Vec<i64> {
  let input = "97
    62
    23
    32
    51
    19
    98
    26
    90
    134
    73
    151
    116
    76
    6
    94
    113
    127
    119
    44
    115
    50
    143
    150
    86
    91
    36
    104
    131
    101
    38
    66
    46
    96
    54
    70
    8
    30
    1
    108
    69
    139
    24
    29
    77
    124
    107
    14
    137
    16
    140
    80
    68
    25
    31
    59
    45
    126
    148
    67
    13
    125
    53
    57
    41
    47
    35
    145
    120
    12
    37
    5
    110
    138
    130
    2
    63
    83
    22
    79
    52
    7
    95
    58
    149
    123
    89
    109
    15
    144
    114
    9
    78";
  let mut adapters = input.split("\n")
      .map(|val| val.trim().parse().unwrap())
      .collect::<Vec<i64>>();
  adapters.push(0);
  adapters.sort();
  adapters.push(adapters.last().unwrap() + 3);
  return adapters;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_jolt_diff_with_all_adapters() {
    let mut test_vec = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
    test_vec.push(0);
    test_vec.sort();
    test_vec.push(test_vec[test_vec.len() - 1] + 3);
    assert_eq!(220, jolt_diff_using_all_adapters(&test_vec));
  }

  #[test]
  fn test_find_all_combos() {
    let mut test_vec = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
    test_vec.push(0);
    test_vec.sort();
    test_vec.push(test_vec[test_vec.len() - 1] + 3);
    let test_vec2 = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
    assert_eq!(8, total_configurations(&test_vec2));
    assert_eq!(19208, total_configurations(&test_vec));
  }
}