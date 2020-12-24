use std::collections::HashMap;

pub fn active_after_6cycles(grid: &HashMap<(i32, i32, i32), char>) -> usize {
  let mut cur = grid.clone();
  for _ in 0..6 {
    cur = cycle3d(&cur);
  }
  return cur.values().filter(|&c| *c == '#').count();
}

pub fn active_after_6cycles_4d(grid: &HashMap<(i32, i32, i32, i32), char>) -> usize {
  let mut cur = grid.clone();
  for _ in 0..6 {
    cur = cycle4d(&cur);
  }
  return cur.values().filter(|&c| *c == '#').count();
}

fn cycle3d(grid: &HashMap<(i32, i32, i32), char>) -> HashMap<(i32, i32, i32), char> {
  let mut largest_x = 0;
  let mut smallest_x = 0;
  let mut largest_y = 0;
  let mut smallest_y = 0;
  let mut largest_z = 0;
  let mut smallest_z = 0;
  for position in grid.keys() {
    let (x,y,z) = *position;
    if x < smallest_x {
      smallest_x = x;
    } else if x > largest_x {
      largest_x = x;
    }

    if y < smallest_y {
      smallest_y = y;
    } else if y > largest_y {
      largest_y = y;
    }

    if z < smallest_z {
      smallest_z = z;
    } else if z > largest_z {
      largest_z = z;
    }
  }

  let mut new_grid = HashMap::new();
  for x in smallest_x - 1..=largest_x + 1 {
    for y in smallest_y - 1..=largest_y + 1 {
      for z in smallest_z - 1..=largest_z + 1 {
        let position = (x,y,z);
        let active_neighbors = get_neighbors3d(&grid, &position).iter()
          .filter(|&c| *c == &'#')
          .count();
        let mut current = *grid.get(&position).unwrap_or(&'.');
        if current == '.' && active_neighbors == 3 {
          current = '#';
        } else if current == '#' && (active_neighbors < 2 || active_neighbors > 3) {
          current = '.';
        }
        new_grid.insert(position, current);
      }
    }
  }

  return new_grid;
}

fn cycle4d(grid: &HashMap<(i32, i32, i32, i32), char>) -> HashMap<(i32, i32, i32, i32), char> {
  let mut largest_x = 0;
  let mut smallest_x = 0;
  let mut largest_y = 0;
  let mut smallest_y = 0;
  let mut largest_z = 0;
  let mut smallest_z = 0;
  let mut largest_w = 0;
  let mut smallest_w = 0;
  for position in grid.keys() {
    let (x,y,z,w) = *position;
    if x < smallest_x {
      smallest_x = x;
    } else if x > largest_x {
      largest_x = x;
    }

    if y < smallest_y {
      smallest_y = y;
    } else if y > largest_y {
      largest_y = y;
    }

    if z < smallest_z {
      smallest_z = z;
    } else if z > largest_z {
      largest_z = z;
    }

    if w < smallest_w {
      smallest_w = w;
    } else if w > largest_w {
      largest_w = w;
    }
  }

  let mut new_grid = HashMap::new();
  for x in smallest_x - 1..=largest_x + 1 {
    for y in smallest_y - 1..=largest_y + 1 {
      for z in smallest_z - 1..=largest_z + 1 {
        for w in smallest_w - 1..=largest_w + 1 {
          let position = (x,y,z,w);
          let active_neighbors = get_neighbors4d(&grid, &position).iter()
            .filter(|&c| *c == &'#')
            .count();
          let mut current = *grid.get(&position).unwrap_or(&'.');
          if current == '.' && active_neighbors == 3 {
            current = '#';
          } else if current == '#' && (active_neighbors < 2 || active_neighbors > 3) {
            current = '.';
          }
          new_grid.insert(position, current);
        }
      }
    }
  }

  return new_grid;
}

fn get_neighbors3d<'a>(grid: &'a HashMap<(i32, i32, i32), char>, position: &(i32, i32, i32)) -> Vec<&'a char> {
  let (x,y,z) = *position;
  let mut values = Vec::new();
  for i in x-1..=x+1 {
    for j in y-1..=y+1 {
      for k in z-1..=z+1 {
        let tuple = (i,j,k);
        if tuple == *position {
          continue;
        }
        values.push(match grid.get(&tuple) {
          Some(val) => val,
          None => &'.',
        });
      }
    }
  }
  return values;
}

fn get_neighbors4d<'a>(grid: &'a HashMap<(i32, i32, i32, i32), char>, position: &(i32, i32, i32, i32)) -> Vec<&'a char> {
  let (x,y,z,w) = *position;
  let mut values = Vec::new();
  for i in x-1..=x+1 {
    for j in y-1..=y+1 {
      for k in z-1..=z+1 {
        for l in w-1..=w+1 {
          let tuple = (i,j,k,l);
          if tuple == *position {
            continue;
          }
          values.push(grid.get(&tuple).unwrap_or(&'.'));
        }
      }
    }
  }
  return values;
}

pub fn parse_input3d(input: &str) -> HashMap<(i32,i32,i32), char> {
  let mut grid = HashMap::new();
  let mut y = 0;
  for line in input.split("\n") {
    let mut x = 0;
    for c in line.trim().chars() {
      grid.insert((x,y,0), c);
      x += 1;
    }
    y += 1;
  }
  return grid;
}

pub fn parse_input4d(input: &str) -> HashMap<(i32,i32,i32, i32), char> {
  let mut grid = HashMap::new();
  let mut y = 0;
  for line in input.split("\n") {
    let mut x = 0;
    for c in line.trim().chars() {
      grid.insert((x,y,0,0), c);
      x += 1;
    }
    y += 1;
  }
  return grid;
}

pub fn read_input() -> String {
  return ".##.####
    .#.....#
    #.###.##
    #####.##
    #...##.#
    #######.
    ##.#####
    .##...#.".to_string();
  
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_neighbor() {
    let mut grid: HashMap<(i32, i32, i32), char> = HashMap::new();
    grid.insert((1,0,0), '#');
    grid.insert((2,1,0), '#');
    grid.insert((0,2,0), '#');
    grid.insert((1,2,0), '#');
    grid.insert((2,2,0), '#');

    assert_eq!(3, get_neighbors3d(&grid, &(2,1,0)).iter().filter(|&c| *c == &'#').count());
  }

  #[test]
  fn first_cycle() {
    let mut grid: HashMap<(i32, i32, i32), char> = HashMap::new();
    grid.insert((1,0,0), '#');
    grid.insert((2,1,0), '#');
    grid.insert((0,2,0), '#');
    grid.insert((1,2,0), '#');
    grid.insert((2,2,0), '#');

    let next = cycle3d(&grid);
    assert_eq!(&'#', next.get(&(0,1,-1)).unwrap());
    assert_eq!(&'.', next.get(&(1,1,0)).unwrap());
    let count = next.values().filter(|&c| *c == '#').count();
    assert_eq!(11, count);
  }

  #[test]
  fn cycles_6() {
    let input = ".#.
      ..#
      ###";
    let grid = parse_input3d(input);
    assert_eq!(112, active_after_6cycles(&grid));
  }

  #[test]
  fn cycles6_4d() {
    let input = ".#.
      ..#
      ###";
    let grid = parse_input4d(input);
    assert_eq!(848, active_after_6cycles_4d(&grid));
  }

}

