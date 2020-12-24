
const DIRECTION_ORDER: [Direction;4] = [Direction::North, Direction::East, Direction::South, Direction::West];

#[derive(Debug, PartialEq)]
pub enum NavInstruction {
  North(i32),
  East(i32),
  South(i32),
  West(i32),
  Forward(i32),
  Right(i32),
  Left(i32)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

impl NavInstruction {
  fn from_input(instruction: &str, value: i32) -> NavInstruction {
    return match instruction {
      "N" => NavInstruction::North(value),
      "E" => NavInstruction::East(value),
      "S" => NavInstruction::South(value),
      "W" => NavInstruction::West(value),
      "F" => NavInstruction::Forward(value),
      "L" => NavInstruction::Left(value),
      "R" => NavInstruction::Right(value),
      _  => panic!("Unrecognized nav instruction {}", instruction),
    }
  } 
}

#[derive(Debug, PartialEq)]
pub struct Ship {
  north: i32,
  south: i32,
  east: i32,
  west: i32,
  current_direction: Direction,
}

impl Ship {
  pub fn new() -> Ship {
    return Ship {
      north: 0,
      south: 0,
      east: 0,
      west: 0,
      current_direction: Direction::East,
    }
  }

  fn turn(&mut self, is_left: bool, amount: i32) {
    let turns = amount / 90;
    let mut dir_index = DIRECTION_ORDER.iter().position(|val| *val == self.current_direction).unwrap() as i32;
    for _ in 0..turns {
      dir_index = if is_left { dir_index - 1 } else { dir_index + 1 };
      if dir_index < 0 {
        dir_index = (DIRECTION_ORDER.len() - 1) as i32;
      } else if dir_index >= DIRECTION_ORDER.len() as i32 {
        dir_index = 0;
      }
    }
    self.current_direction = DIRECTION_ORDER[dir_index as usize].clone();
  }

  fn go_forward(&mut self, amount: i32) {
    match self.current_direction {
      Direction::East => self.east += amount,
      Direction::North => self.north += amount,
      Direction::West => self.west += amount,
      Direction::South => self.south += amount,
    }
  }

  pub fn execute_instruction(&mut self, instruction: &NavInstruction) {
    match instruction {
      NavInstruction::North(value) => self.north += value,
      NavInstruction::East(value) => self.east += value,
      NavInstruction::South(value) => self.south += value,
      NavInstruction::West(value) => self.west += value,
      NavInstruction::Forward(value) => self.go_forward(*value),
      NavInstruction::Right(value) => self.turn(false, *value),
      NavInstruction::Left(value) => self.turn(true, *value),
    }
  }

  pub fn manhattan_position(&self) -> i32 {
    return (self.north - self.south).abs() + (self.east - self.west).abs();
  }
}

#[derive(Debug, PartialEq)]
pub struct WaypointShip {
  north: i32,
  south: i32,
  east: i32,
  west: i32,
  waypoint_1: (Direction, i32),
  waypoint_2: (Direction, i32),
}

impl WaypointShip {
  pub fn new() -> WaypointShip {
    return WaypointShip {
      north: 0,
      south: 0,
      east: 0,
      west: 0,
      waypoint_1: (Direction::North, 1),
      waypoint_2: (Direction::East, 10),
    }
  }

  fn turn(&mut self, is_left: bool, amount: i32) {
    let turns = amount / 90;
    self.waypoint_1 = (self.turn_waypoint(&self.waypoint_1.0, is_left, turns), self.waypoint_1.1);
    self.waypoint_2 = (self.turn_waypoint(&self.waypoint_2.0, is_left, turns), self.waypoint_2.1);
  }

  fn turn_waypoint(&self, starting_dir: &Direction, is_left: bool, turns: i32) -> Direction {
    let mut dir_index = DIRECTION_ORDER.iter().position(|val| val == starting_dir).unwrap() as i32;
    for _ in 0..turns {
      dir_index = if is_left { dir_index - 1 } else { dir_index + 1 };
      if dir_index < 0 {
        dir_index = (DIRECTION_ORDER.len() - 1) as i32;
      } else if dir_index >= DIRECTION_ORDER.len() as i32 {
        dir_index = 0;
      }
    }
    return DIRECTION_ORDER[dir_index as usize].clone();
  }

  fn go_forward(&mut self, amount: i32) {
    let waypoints = vec![&self.waypoint_1, &self.waypoint_2];
    for waypoint in waypoints {
      match waypoint {
        (Direction::East, val) => self.east += amount * val,
        (Direction::North, val) => self.north += amount * val,
        (Direction::West, val) => self.west += amount * val,
        (Direction::South, val) => self.south += amount * val,
      }
    }
  }

  fn move_waypoint_ns(&mut self, amount: i32) {
    self.waypoint_1 = self.calculate_waypoint_ns(&self.waypoint_1, amount);
    self.waypoint_2 = self.calculate_waypoint_ns(&self.waypoint_2, amount);
  }

  fn calculate_waypoint_ns(&self, waypoint: &(Direction, i32), amount: i32) -> (Direction, i32) {
    let mut waypoint_val = waypoint.1;
    let mut waypiont_dir = waypoint.0.clone();
    if waypoint.0 == Direction::North {
      waypoint_val += amount;
      if waypoint_val < 0 {
        waypoint_val = waypoint_val.abs();
        waypiont_dir = Direction::South;
      }
    } else if waypoint.0 == Direction::South {
      waypoint_val -= amount;
      if waypoint_val < 0 {
        waypoint_val = waypoint_val.abs();
        waypiont_dir = Direction::North;
      }
    }
    return (waypiont_dir, waypoint_val);
  }

  fn move_waypoint_ew(&mut self, amount: i32) {
    self.waypoint_1 = self.calculate_waypoint_ew(&self.waypoint_1, amount);
    self.waypoint_2 = self.calculate_waypoint_ew(&self.waypoint_2, amount);
  }

  fn calculate_waypoint_ew(&self, waypoint: &(Direction, i32), amount: i32) -> (Direction, i32) {
    let mut waypoint_val = waypoint.1;
    let mut waypiont_dir = waypoint.0.clone();
    if waypoint.0 == Direction::East {
      waypoint_val += amount;
      if waypoint_val < 0 {
        waypoint_val = waypoint_val.abs();
        waypiont_dir = Direction::West;
      }
    } else if waypoint.0 == Direction::West {
      waypoint_val -= amount;
      if waypoint_val < 0 {
        waypoint_val = waypoint_val.abs();
        waypiont_dir = Direction::East;
      }
    }
    return (waypiont_dir, waypoint_val);
  }

  pub fn execute_instruction(&mut self, instruction: &NavInstruction) {
    match instruction {
      NavInstruction::North(value) => self.move_waypoint_ns(*value),
      NavInstruction::East(value) => self.move_waypoint_ew(*value),
      NavInstruction::South(value) => self.move_waypoint_ns(-value),
      NavInstruction::West(value) => self.move_waypoint_ew(-value),
      NavInstruction::Forward(value) => self.go_forward(*value),
      NavInstruction::Right(value) => self.turn(false, *value),
      NavInstruction::Left(value) => self.turn(true, *value),
    }
  }

  pub fn manhattan_position(&self) -> i32 {
    return (self.north - self.south).abs() + (self.east - self.west).abs();
  }
}

pub fn navigate_and_get_position(instructions: &Vec<NavInstruction>) -> i32 {
  let mut ship = Ship::new();
  for instruction in instructions {
    ship.execute_instruction(instruction);
  }
  return ship.manhattan_position();
}

pub fn naviage_using_waypoint(instructions: &Vec<NavInstruction>) -> i32 {
  let mut ship = WaypointShip::new();
  for instruction in instructions {
    ship.execute_instruction(instruction);
  }
  return ship.manhattan_position();
}

pub fn parse_instructions(input: &str) -> Vec<NavInstruction>{
  let mut instructions = Vec::new();
  for line in input.split("\n") {
    let mut line_iter = line.trim().chars();
    let direction = line_iter.next().unwrap().to_string();
    let value = line_iter.collect::<String>().parse().unwrap();
    instructions.push(NavInstruction::from_input(&direction, value));
  }

  return instructions;
}

pub fn read_input() -> String {
  return "W5
    F66
    S4
    E1
    F78
    L90
    F79
    S4
    F64
    R180
    F24
    N4
    L90
    N3
    R180
    E5
    N5
    F68
    E3
    L180
    F56
    E3
    S5
    F75
    E1
    L90
    F53
    S2
    E3
    S4
    L180
    W4
    L90
    S1
    F51
    L90
    W3
    L90
    W3
    F39
    W2
    R90
    E1
    R90
    W4
    R180
    N1
    E5
    R90
    F63
    L90
    N4
    R180
    N3
    F25
    W3
    L90
    N4
    W3
    S5
    E3
    R90
    E5
    F17
    S5
    F20
    E2
    L90
    E1
    S5
    R90
    F13
    E3
    N4
    W1
    L90
    N3
    F95
    N1
    F37
    N2
    E5
    L90
    W2
    N5
    F8
    S4
    E4
    L90
    E1
    F92
    S2
    F26
    S1
    F97
    E4
    L90
    W1
    R270
    F28
    R90
    S3
    N2
    F93
    N1
    E4
    S1
    E5
    S1
    W2
    F70
    N4
    W5
    F74
    N1
    L90
    S2
    L180
    S1
    E3
    R180
    W3
    N5
    L90
    W3
    F20
    E5
    R90
    E2
    F52
    L270
    E3
    R90
    E3
    R90
    F8
    E5
    F26
    E1
    F87
    N1
    F77
    W4
    F60
    S2
    F39
    R90
    S1
    F5
    N4
    F98
    W5
    R90
    W1
    R90
    S4
    L90
    N5
    F19
    S2
    L270
    F44
    E1
    F90
    R90
    F11
    S4
    R90
    E2
    L90
    N4
    R180
    E5
    F66
    S5
    F11
    L180
    E1
    N1
    L180
    S3
    E5
    N2
    W5
    F75
    E3
    N1
    W5
    L90
    S5
    R90
    S5
    F2
    W2
    N4
    F15
    S5
    L90
    E1
    F49
    W5
    F48
    R90
    N4
    W5
    L90
    E4
    L270
    F77
    E2
    F33
    E1
    N2
    L90
    F75
    E5
    L90
    F16
    L90
    F82
    E4
    S5
    F72
    S3
    R270
    F88
    W1
    L90
    N5
    L180
    W1
    R90
    E1
    F39
    R90
    F27
    R90
    W3
    R90
    F55
    R180
    W5
    S2
    F18
    E3
    F1
    E5
    N1
    W4
    F10
    L90
    F76
    N4
    F40
    R180
    E4
    R90
    F69
    W5
    R90
    W3
    F67
    W5
    N4
    E5
    N4
    W2
    L90
    E5
    F26
    R180
    F48
    W3
    E2
    F84
    E1
    R90
    F51
    L180
    S3
    W5
    R90
    N1
    F69
    W2
    S5
    R180
    F57
    W2
    F45
    R90
    N2
    L90
    N4
    R180
    F8
    L180
    F48
    W5
    L90
    S5
    F52
    L90
    S5
    R90
    E4
    L90
    N4
    L90
    W5
    F80
    R90
    W1
    F38
    R90
    F25
    W1
    F31
    N4
    S4
    F91
    R180
    S2
    L90
    F30
    S3
    W3
    S4
    W1
    N4
    F25
    R90
    W3
    F10
    N3
    R90
    N3
    L90
    E1
    S4
    R90
    L90
    F36
    E4
    R180
    F27
    R270
    F57
    L180
    F72
    S2
    R90
    W1
    F19
    S3
    F27
    N4
    W5
    R90
    F42
    L180
    F57
    E5
    F63
    N3
    F97
    L90
    S5
    F11
    L90
    W1
    F71
    W3
    R90
    W4
    F43
    F71
    W3
    F20
    N1
    F75
    E5
    F15
    N3
    F3
    W2
    F13
    W5
    F88
    E5
    L90
    N4
    L90
    F11
    L90
    F58
    W2
    R90
    S5
    F55
    S4
    F83
    F8
    N1
    R90
    F36
    W2
    F57
    N4
    L180
    W2
    F37
    S1
    F18
    E1
    F82
    S5
    E4
    R90
    E2
    S5
    R90
    S5
    R180
    F91
    E2
    N3
    F43
    E2
    R90
    S4
    E1
    R90
    S3
    L180
    F48
    F57
    E5
    F87
    S4
    R90
    N4
    E2
    N4
    W5
    R270
    F31
    N5
    W5
    N1
    F92
    S5
    F40
    W3
    F79
    L180
    E5
    F83
    L180
    N1
    F30
    N2
    E1
    S3
    L90
    E5
    F56
    R180
    E4
    F17
    W4
    L180
    S5
    E1
    F57
    E3
    F99
    S3
    F29
    L90
    F61
    S5
    W2
    S3
    F83
    R180
    F83
    E1
    R90
    E4
    N3
    W5
    N3
    N2
    F21
    L90
    L90
    F72
    S3
    L90
    E3
    F16
    R180
    F75
    S3
    R90
    S3
    L90
    F82
    R90
    W5
    R90
    N4
    R90
    F14
    N1
    F59
    E5
    S2
    W3
    N3
    S5
    E4
    F43
    E2
    F31
    S2
    F59
    N2
    R90
    S3
    L90
    N3
    F88
    N2
    F22
    N4
    L90
    N2
    L90
    F21
    W4
    F97
    R90
    F29
    S5
    W4
    F40
    N3
    L90
    F63
    N5
    F56
    R90
    L90
    S3
    R90
    S5
    F53
    N1
    W3
    R90
    F1
    S2
    E4
    N4
    F68
    R90
    W3
    S5
    S1
    E4
    L180
    F8
    E5
    L180
    W5
    N1
    F42
    S3
    F61
    L90
    S3
    F29
    S4
    E3
    F6
    R90
    N2
    L180
    W3
    F48
    S5
    E3
    R90
    E4
    R180
    F87
    L90
    F73
    S2
    W1
    S5
    E5
    S4
    E2
    S4
    F93
    L90
    W3
    S3
    F17
    N5
    F42
    S5
    R90
    E1
    S2
    F7
    S3
    W2
    L180
    W4
    F99
    E5
    S5
    F93
    L270
    F20
    N4
    L90
    F1
    N1
    L90
    S2
    F96
    L180
    S2
    F8
    L90
    S4
    R90
    F70
    W1
    R180
    S2
    R180
    N3
    L90
    E4
    R90
    F40
    E1
    S4
    R90
    W2
    R90
    L90
    N5
    L180
    F5
    E5
    S3
    F43
    E2
    E2
    F79
    E5
    F66
    E5
    S4
    E5
    S4
    W2
    S2
    F41
    L90
    N5
    F60
    W5
    N4
    E2
    S5
    R90
    W2
    F98
    W3
    S2
    W5
    L90
    W5
    F16
    S5
    L90
    F85
    R90
    F56
    W4
    R90
    E1
    R90
    F5
    N5
    E5
    R90
    E4
    F77
    S1
    E5
    F24
    L180
    W1
    S3
    F31
    E5
    S5
    W4
    F83
    W3
    N4
    L90
    N3
    L90
    E2
    R180
    S1
    F17
    R90
    R180
    S4
    F100
    E2
    L180
    N2
    L90
    E3
    N5
    W2
    F19
    W4
    R180
    F56
    R90
    W1
    R90
    E4
    S2
    R90
    F10
    E4
    E2
    F61
    E1
    S3
    F33
    N4
    F95
    S3
    F84
    L180
    S1
    L90
    N1
    E3
    F40
    L90
    E3
    S2
    F56
    L180
    W5
    S5
    R90
    E3
    N1
    F60
    L90
    F62".to_string();
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_nav() {
    let input = "F10
      N3
      F7
      R90
      F11";
    let instructions = parse_instructions(input);
    assert_eq!(25, navigate_and_get_position(&instructions));
  }

  #[test]
  fn test_waypoint() {
    let input = "F10
      N3
      F7
      R90
      F11";
    let instructions = parse_instructions(input);
    assert_eq!(286, naviage_using_waypoint(&instructions));
  }
}