use std::cmp;

#[derive(Debug, PartialEq, Clone)]
pub enum Seat {
  Floor,
  Empty,
  Occupied,
}

pub fn count_stable_occupied(seats: &Vec<Vec<Seat>>) -> usize {
  let mut current_setup = seats.clone();
  let mut next_setup = seat_round(&current_setup);
  while current_setup != next_setup {
    current_setup = next_setup;
    next_setup = seat_round(&current_setup);
  }
  return next_setup.iter().flat_map(|row| row)
    .filter(|&seat| *seat == Seat::Occupied)
    .count();
}

fn seat_round(current_setup: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
  let mut next_setup = Vec::new();
  for row in 0..current_setup.len() {
    let mut next_row = Vec::new();
    for col in 0..current_setup[row].len() {
      let adjacent_seats = get_adjacent_seats(row, col, &current_setup);
      let occupied_adjacent: i32 = adjacent_seats.iter()
        .map(|&seat| if *seat == Seat::Occupied {1} else {0})
        .sum();
      let next_seat = match current_setup[row][col] {
        Seat::Floor => Seat::Floor,
        Seat::Empty => if occupied_adjacent == 0 { Seat::Occupied } else { Seat::Empty },
        Seat::Occupied => if occupied_adjacent >=4 { Seat::Empty } else { Seat::Occupied },
      };
      next_row.push(next_seat);
    }
    next_setup.push(next_row);
  }
  return next_setup;
}

pub fn count_stable_los(seats: &Vec<Vec<Seat>>) -> usize {
  let mut current_setup = seats.clone();
  let mut next_setup = seat_rount_los(&current_setup);
  while current_setup != next_setup {
    current_setup = next_setup;
    next_setup = seat_rount_los(&current_setup);
  }
  return next_setup.iter().flat_map(|row| row)
    .filter(|&seat| *seat == Seat::Occupied)
    .count();
}

fn seat_rount_los(current_setup: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
  let mut next_setup = Vec::new();
  for row in 0..current_setup.len() {
    let mut next_row = Vec::new();
    for col in 0..current_setup[row].len() {
      let adjacent_seats = get_adjacent_seats_los(row, col, &current_setup);
      let occupied_adjacent: i32 = adjacent_seats.iter()
        .map(|&seat| if *seat == Seat::Occupied {1} else {0})
        .sum();
      let next_seat = match current_setup[row][col] {
        Seat::Floor => Seat::Floor,
        Seat::Empty => if occupied_adjacent == 0 { Seat::Occupied } else { Seat::Empty },
        Seat::Occupied => if occupied_adjacent >=5 { Seat::Empty } else { Seat::Occupied },
      };
      next_row.push(next_seat);
    }
    next_setup.push(next_row);
  }
  return next_setup;
}

// My first use of lifetimes!
fn get_adjacent_seats<'a>(row: usize, col: usize, seats: &'a Vec<Vec<Seat>>) -> Vec<&'a Seat> {
  let mut adjacent_seats = Vec::new();
  // To for loops with uzise bounds (can't be less than 0) are a problem
  for r in cmp::max(0, row as i32 - 1) as usize..=(row + 1) {
    for c in cmp::max(0, col as i32 - 1 ) as usize..=(col + 1) {
      if !(r == row && c == col) && r < seats.len() && c < seats[r].len() {
        adjacent_seats.push(&seats[r][c]);
      }
    }
  }
  return adjacent_seats;
}

fn get_adjacent_seats_los<'a>(row: usize, col: usize, seats: &'a Vec<Vec<Seat>>) -> Vec<&'a Seat> {
  let mut adjacent_seats = Vec::new();
  
  let mut diag_up_left_row = row;
  let mut diag_up_left_col = col;
  let mut diag_up_left_seat = &Seat::Floor;
  while diag_up_left_col > 0 && diag_up_left_row > 0 && *diag_up_left_seat == Seat::Floor {
    diag_up_left_col -= 1;
    diag_up_left_row -= 1;
    diag_up_left_seat = &seats[diag_up_left_row][diag_up_left_col];
  }
  adjacent_seats.push(diag_up_left_seat);

  let mut left_col = col;
  let mut left_seat = &Seat::Floor;
  while left_col > 0 && *left_seat == Seat::Floor {
    left_col -= 1;
    left_seat = &seats[row][left_col];
  }
  adjacent_seats.push(left_seat);

  let mut diag_down_left_row = row;
  let mut diag_down_left_col = col;
  let mut diag_down_left_seat = &Seat::Floor;
  while diag_down_left_col > 0 && diag_down_left_row < seats.len() - 1 && *diag_down_left_seat == Seat::Floor {
    diag_down_left_col -= 1;
    diag_down_left_row += 1;
    diag_down_left_seat = &seats[diag_down_left_row][diag_down_left_col];
  }
  adjacent_seats.push(diag_down_left_seat);

  let mut down_row = row;
  let mut down_seat = &Seat::Floor;
  while down_row < seats.len() - 1 && *down_seat == Seat::Floor {
    down_row += 1;
    down_seat = &seats[down_row][col];
  }
  adjacent_seats.push(down_seat);

  let mut diag_down_right_row = row;
  let mut diag_down_right_col = col;
  let mut diag_down_right_seat = &Seat::Floor;
  while diag_down_right_row < seats.len() - 1 && diag_down_right_col < seats[row].len() - 1 && *diag_down_right_seat == Seat::Floor {
    diag_down_right_row += 1;
    diag_down_right_col += 1;
    diag_down_right_seat = &seats[diag_down_right_row][diag_down_right_col];
  }
  adjacent_seats.push(diag_down_right_seat);

  let mut right_col = col;
  let mut right_seat = &Seat::Floor;
  while right_col < seats[row].len() -1 && *right_seat == Seat::Floor {
    right_col += 1;
    right_seat = &seats[row][right_col];
  }
  adjacent_seats.push(right_seat);

  let mut diag_up_right_row = row;
  let mut diag_up_right_col = col;
  let mut diag_up_right_seat = &Seat::Floor;
  while diag_up_right_row > 0 && diag_up_right_col < seats[row].len() - 1 && *diag_up_right_seat == Seat::Floor {
    diag_up_right_row -= 1;
    diag_up_right_col += 1;
    diag_up_right_seat = &seats[diag_up_right_row][diag_up_right_col];
  }
  adjacent_seats.push(diag_up_right_seat);

  let mut up_row = row;
  let mut up_seat = &Seat::Floor;
  while up_row > 0 && *up_seat == Seat::Floor {
    up_row -= 1;
    up_seat = &seats[up_row][col];
  }
  adjacent_seats.push(up_seat);

  return adjacent_seats;
}

pub fn read_input() -> String {
  return "LLLLLL.LL.LLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLL.LLLL.L.LLL.LLLLLLLL.LLLLLLLL.LLLLL
    LLLLLL.L.LL.LLLL.L.LLLL.LLLLLLL.LLLLL.LLLLLLLL.LLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLL
    LLLLLL.LLLL.LLLL.LLLLLLLLLLLLLL.LLLLL.LLLL.LLLLLLLLLLL.L.LLLL.LLLLL.LLL..LLL.LLLLLLLLLLLLLL
    LLLLLL.LLLL.LLLLLLLLLL.L.LLLLLLLLLLLLLLLLLLL.L.LLLLLLL.L.L.LL.LLLLL..LLLLLLL.LLLLLLLL.LL.LL
    LLLLLL.LLLL.LLLL.LLLLLLLLLLLLL..LLLLLLLLLLLLLL.LLLL.LL.LLLLLL.LLLLL.LLL.LLLL.LLLLLLLL.LLLLL
    LLLL.L.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLL.L..LLLLL.LLLLLLLL.L.LLLLLL.LLLLL
    LLLLLL..LLL.LLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLL.L.LLLL.LLLLLL.LL.LLLLL.LLLLLLLLLLLLLLLLL.LLLLL
    LLLLLL.LLLLLLLLLLLL.LLLL.LLLLLL.LLLLL.LL.LLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLLL..LLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLL.LLLLL
    .L.L......LL..LL.....L..L...L...L.L...L.LL.....LL..L..L....L...L....L......L.L.L...L.LLLL..
    LLLLLLLLL.L.LLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLLLLLLLLLLLL.LLLLLLL.LLLL.L.LLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLL..LLLLLLLL.LLLLLLLL.LLLLL
    LL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LL.LLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLLL
    .LLLLL.LLLL.LLLL..LLLLLL.L.LLLLLL.LLL.LLLLL.LL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLL
    LLLLLL.LLLL.LL.LLLLLLLL.LLLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLL.LLL.LLLLL
    LLLLL.LLLLLLL.LL.LLLLLL..LLLLLL..LLLL.LLLLLLLL.L.LLL.L.LLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLL
    LLLLLLLLLLL.LLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLL.LLLLLLLLLLL.LLLLL
    .LLLLL.LLLL.L.LL..LLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLL.L.LLLLLLLLLLLLLLLLLLLLLLL.LLLLL
    LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLL.L.LLLL.LLLLL.LLLLLLLL..LLLLLLL.LLLLL
    .L...LL.LLL....LL..LL...L..LLL.LL...L...L........L....LL..L...L.....LL......L...L..LL....L.
    LL.LLL.LLLLLLLLL.LLLLLLL.LLLLLL.L.LLL.LLLLLLLL.LLLLLL..LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL
    LLLLLLLLLLL.LLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLL.LLLLLLL..LLLLLLL...LLLL
    LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLL..LLLLLLL.LLLLLLLL.LLLLL
    LLL.LL.LLLLLLL.LLL.LLLLL.LLLLLLLLLLLL.LL.LLLLL..LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL
    LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLL.LLLLLLLLL.LLLLL
    L..LLL.LLLL.LLLLLLLLLLLL.LL.LLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL
    LLLLLL.LLLL.LLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLL.LLL.LLLL.
    LLLLLL.LLL..LLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLL.LLLLLLL.LLLLLL.LLLLL.LLLL.LLL.LLLLLLLL.LLLLL
    ..LL...LLL....L....L.LLL.L.L...L.LLL..L...L..L..L........L.LL.L.L.LLLLLL...L.L.LL.L....LL..
    LLLLLL.LLLLLLLLL.LL.L.LL.LLLLLL.L.LLL..LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLL..LL.LLLLLLLL.LLLL.LLLLLL.LLLLL.L.LLL.LL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.L..LL
    LLLLLL.LLLLLLLLL.LL.LLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLL.L.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLL.LLLLL
    LLLLLL.LLLL.LLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLL.LL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLL.LL...LLLL.LLLLLLL.L.LLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLLLL
    L.L.LL.....L.......L.....LLL..L......LL.LLL....L.L.L..........L.LL..LLL..L....LL..L..L..L.L
    LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLL.LLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL
    LLLLLL.LLLL.LLLLLLLLLLLL..LLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LL.LL.LLLLLLLLLLLLLLLLL.LL.LL
    LLLLLLLLLLL.LLLL.L.LLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LL.LLLLLLLLLL..LLLLLLLL.LLLLL
    LLLLLLLLLLLLLLLL.LL.LLLLLLLLLLL..LLLL.LLLLLL.L.LLLLLLL.LLLLL..LLLLLLLLLL.LLL.LLLL.LLLLLLLLL
    LLLLLL.LLLL.LLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLL.L
    LLLLL.LLLLLLLLLLLLLLL.LL.LLLL.L.LLLLL.LLLLLLLLLLLLLLLL.LLLLLL.L.LLLLLLLLLLLL.LLLLLLLL.LLLLL
    ..LLLL.L...LLL.L......L....L.L..L......LL..L.LL.LL.LL................LL.LL....L.........L..
    LLLLLL.LLLL.LLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLL.LLLLLLLLL..LLLL
    LLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL..LLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.L.LL.LLL.LLL.L
    LLLLLL.LLLL.LLLL.LLLLLLL.LLLLLL.LLLLLLL.LLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
    LLLLLL.LLLL.LL.L.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.L.L.L.LLLLLLLL.LL.LLLLLLLL.LL
    LLL.LL.LLLL.LLLL.LLLLLLL.LLLLL..LLLLL.LLLL.LLL.LLLLLLLLLLLLL..LLLLLLLLLLLLLLLLLLLLLLL.LLLLL
    LLLLLLLLLLL.LLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLL..LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    L.LLLL.LLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLL.L..LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLL
    ..L..L.L.LLLL.....LLL.LLL....L..LL.L..L......L.............LL....LL..L.L..L...L.....L.L...L
    LLLLLL.LLLL.L.LL.L.LLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLL.LLLLLLLLLLLL
    LLLLLL..LLL.LLLL.LLLLLLL.LLLLLL.LLLLL.LLLL.LLL.LLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL
    LLLLLL.LLLL..LLLLLLLLLLL.LLLLLL.L.LLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LL.LLL.L.LLLLLLLL.LLLLL
    LLLLLL.LLLLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLL.L.LLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
    LLLLLL.LL.L.LL.L.LLLLLLLLL.LLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL
    LLLLLLLLL.L.LLLLLLLLLL.L.LLLLL.LLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLL.LLLLLLLLLLLL
    LLLLLL.LLLLLL.LL.LLLLLLLL.LLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLL.
    LLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLL.LLLLLLLL.LLLL
    ....L..L..LL....L.L.LLL.L.L...L...L.L.L..L..L...LLL.......L.......LL..L...LL..L.L....LL.LL.
    LLLLLL.LLLL.LL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
    LLLLLL.LLLL.LLLL..LLLLLL.LLLL.L.LLLLL.LLL.LLLL.LLLLLLL.LLLLLLLLLLLL.L.LLLLLL.LLLLLLLLLLLLLL
    LLLLLL.LLLL.LLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.L.LLLLLL..LLLL
    LLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLL.L.LLLL.LLLLL.LLLLL.LL.LLLLLLLL.LLLLL
    .LLLLL.LLLL.LLLLLLLLLLLL.L.LLLLLLLLLL.LLLLLLLL.LLLLLLLLLLL.LL.L.LLLLLLLLLLLL.LLLLL.LL.LLLLL
    LL.LLL.LLLL.LLLL.LLLLLLLLLLLLLL.LLLLL.LLLL.LLL.L.L.LLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLL.L.LLLLL
    LLLLLLLLLLLLLL.L.LLLLLLL.LLLLLL.LLLLL.LLL.LLLLLLLLLLLL.L.LLLL.LLLLL.LLLLLLLLLLLLLLLLL.LLLL.
    LLLLLL.L.LL.LLLLLLLLLLLL.LL.LLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLL.L.LLL.LLLLLLLL.LLLLLLL.LLLLLL
    LLLLLLLLLLL.LLLLLLLLLLLL.L.L.LL.LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLL.LLL.LLLLL.LLLLLLLLLLL.LLLLL
    .L...LL..LL....L...L.....LLLL..L....L..LL...L...LLL...L.LLL..L.LL.L.L.LL.L..LL.....LLLL.LL.
    LLLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLL..LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLLLLLLL.L.LLLLLL.LLLLLLLLLL.L.LLL.LLLL.LLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLLL.LLLLLLL.LLLLL
    LLLLLL.LLLL.LLLL.LLLLLLL.LL.LLLLLLLLL.LLLLLLL..LLLLLLL.LLL.LL.LLLLL.LLLLL.LL.LLLLLL.L.LLLLL
    LLLLLL.LLLL.LLLL.LLLLLLLLLLL.LL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLLLLLLL.LLLLLLLLLLLL.LLLLLL.LLLLL..LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL..LLLLLLL.LLLLL
    LLL.LLLLLLL.LLLL.LLLLLLL.LLLLLLLLL.LL.LLLLLLLL.LLLLLLL..LLLLL.LLLLLLLLLLL.LL.LLLLLLLLLLLLL.
    LLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLL.LLL.LLLLLLLLLLLLL
    ...L...L.L......L..L..L.LL...L..........L..L.L.L........L..L..L.L.LL..LL..LL.........L.....
    LLLL.L.LLLL.LLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
    LLLLLLLLLLL.LLLLL.LLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLL.LLLLLLLLLLLLLLLLLL
    LLLLLL.LLLLLLLLLLLLLLLLL.LLLLLL.L.LLL.LL.LLLLLLLLLLLL..LLLLL..LLLLL.LLLLLLL..LLLLLLLLLLLLLL
    LLLLLL.LLLL.LLLL.LLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLL.LLLLL
    .LLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLL.LLLLL
    L.LL.LLLLLL.LL.L..LLLLLL.LLLLLL.LLLLLLLLLLLLLL.LL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLL.LLL.LLLLL
    L..L...L..L...LL...L.L.L..LLL.LL...L......LLL...L.L....L.......L...L.LL.L....L....L...L....
    LLLLLL.LLLL.LLLL.L.LLLLL.LLL.LL.LLLLL.LLLL.LLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLL
    LLL.LL.LLLL.LLL...L.LLL..LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLL.LLLLL
    LLLLLL.LLLL.LLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLLLL
    LLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLL.LLL.LLLLLLLLLLLLL.LL.LLLLL.LLLLLLL.LLLLL
    LLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLL
    LLLLLLLLLLL.LLLL.LLLLLLL.LLLLLL..LLLL.LLLLLLLL.LLLLLLLLL.LLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLL.
    LLLLLL.LLLLLLLLLLLLLL.LL.L.LLLLLLLLLL.LLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLL.L
    LLLLLL.LLLLLLLLL.LLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLLLLLLLLLLLLL.LLLLL.LLLLLL.L.LLLLLLLLLLLLLL
    LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLLLL.LLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLL".to_string();
}

pub fn parse_seating(input: &str) -> Vec<Vec<Seat>> {
  let mut seats = Vec::new();
  for line in input.split("\n") {
    let mut row = Vec::new();
    for item in line.trim().chars(){
      if item == 'L' {
        row.push(Seat::Empty);
      } else if item == '.' {
        row.push(Seat::Floor);
      } else {
        panic!("unexpected input seat {}", item);
      }
    }
    seats.push(row);
  }
  return seats;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vector_eqaul() {
    let vec1 = vec![Seat::Floor, Seat::Empty, Seat::Occupied];
    let vec2 = vec![Seat::Floor, Seat::Empty, Seat::Occupied];
    let vec3 = vec![Seat::Floor, Seat::Empty, Seat::Empty];
    assert_eq!(vec1, vec2);
    assert_ne!(vec2, vec3);

    let vec4 = vec![vec1, vec3];
    let vec5 = vec![
        vec![Seat::Floor, Seat::Empty, Seat::Occupied],
        vec![Seat::Floor, Seat::Empty, Seat::Empty]];
    assert_eq!(vec4, vec5);
  }

  #[test]
  fn adjacent_seats() {
    let seats = vec![
        vec![Seat::Floor, Seat::Empty, Seat::Occupied],
        vec![Seat::Floor, Seat::Empty, Seat::Empty],
        vec![Seat::Floor, Seat::Floor, Seat::Occupied]];

    assert_eq!(vec![&Seat::Floor, &Seat::Empty, &Seat::Occupied, &Seat::Floor, &Seat::Empty, &Seat::Floor, &Seat::Floor, &Seat::Occupied], get_adjacent_seats(1, 1, &seats));
    assert_eq!(vec![&Seat::Empty, &Seat::Floor, &Seat::Empty], get_adjacent_seats(0, 0, &seats));
    assert_eq!(vec![&Seat::Empty, &Seat::Occupied, &Seat::Empty, &Seat::Floor, &Seat::Occupied], get_adjacent_seats(1, 2, &seats));
  }

  #[test]
  fn test_stable_seating() {
    let input = 
      "L.LL.LL.LL
      LLLLLLL.LL
      L.L.L..L..
      LLLL.LL.LL
      L.LL.LL.LL
      L.LLLLL.LL
      ..L.L.....
      LLLLLLLLLL
      L.LLLLLL.L
      L.LLLLL.LL";
    let seats = parse_seating(input);
    assert_eq!(37, count_stable_occupied(&seats));
  }

  #[test]
  fn light_of_sight_adjacetn() {
    let seats = vec![
        vec![Seat::Floor, Seat::Empty, Seat::Occupied],
        vec![Seat::Floor, Seat::Floor, Seat::Empty],
        vec![Seat::Empty, Seat::Floor, Seat::Occupied]];
    assert_eq!(vec![&Seat::Floor, &Seat::Floor, &Seat::Floor, &Seat::Empty, &Seat::Occupied, &Seat::Empty, &Seat::Floor, &Seat::Floor], get_adjacent_seats_los(0, 0, &seats));
    assert_eq!(vec![&Seat::Empty, &Seat::Floor, &Seat::Floor, &Seat::Occupied, &Seat::Floor, &Seat::Floor, &Seat::Floor, &Seat::Occupied], get_adjacent_seats_los(1, 2, &seats));
  }

  #[test]
  fn test_los_seating() {
    let input = 
      "L.LL.LL.LL
      LLLLLLL.LL
      L.L.L..L..
      LLLL.LL.LL
      L.LL.LL.LL
      L.LLLLL.LL
      ..L.L.....
      LLLLLLLLLL
      L.LLLLLL.L
      L.LLLLL.LL";
    let seats = parse_seating(input);
    assert_eq!(26, count_stable_los(&seats));
  }

}