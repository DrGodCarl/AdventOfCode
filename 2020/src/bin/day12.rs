use anyhow::Result;
use utils::read_lines;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
enum Instr {
    #[display("N{0}")]
    North(isize),
    #[display("S{0}")]
    South(isize),
    #[display("E{0}")]
    East(isize),
    #[display("W{0}")]
    West(isize),
    #[display("L{0}")]
    Left(isize),
    #[display("R{0}")]
    Right(isize),
    #[display("F{0}")]
    Forward(isize),
}

fn move_y(point: &(isize, isize), y: isize) -> (isize, isize) {
    (point.0, point.1 + y)
}

fn move_x(point: &(isize, isize), x: isize) -> (isize, isize) {
    (point.0 + x, point.1)
}

struct State {
    bearing: isize,
    position: (isize, isize),
    waypoint: Option<(isize, isize)>,
}

impl State {
    fn new() -> Self {
        State {
            bearing: 0,
            position: (0, 0),
            waypoint: None,
        }
    }

    fn with_waypoint(waypoint: (isize, isize)) -> Self {
        State {
            bearing: 0,
            position: (0, 0),
            waypoint: Some(waypoint),
        }
    }

    fn move_toward_waypoint(&self, times: isize) -> Self {
        let position = match self.waypoint {
            Some((x, y)) => (self.position.0 + x * times, self.position.1 + y * times),
            None => self.position,
        };
        State {
            bearing: self.bearing,
            position,
            waypoint: self.waypoint,
        }
    }

    fn move_y(&self, y: isize) -> Self {
        State {
            bearing: self.bearing,
            position: move_y(&self.position, y),
            waypoint: self.waypoint,
        }
    }

    fn move_x(&self, x: isize) -> Self {
        State {
            bearing: self.bearing,
            position: move_x(&self.position, x),
            waypoint: self.waypoint,
        }
    }

    fn rotate(&self, deg: isize) -> Self {
        State {
            bearing: (self.bearing + deg).rem_euclid(360),
            position: self.position,
            waypoint: self.waypoint,
        }
    }

    fn move_waypoint_y(&self, y: isize) -> Self {
        State {
            bearing: self.bearing,
            position: self.position,
            waypoint: self.waypoint.map(|w| move_y(&w, y)),
        }
    }

    fn move_waypoint_x(&self, x: isize) -> Self {
        State {
            bearing: self.bearing,
            position: self.position,
            waypoint: self.waypoint.map(|w| move_x(&w, x)),
        }
    }

    fn rotate_waypoint(&self, deg: isize) -> Self {
        let deg = deg.rem_euclid(360);
        State {
            bearing: self.bearing,
            position: self.position,
            waypoint: self.waypoint.map(|(x, y)| match deg {
                0 => (x, y),
                90 => (-y, x),
                180 => (-x, -y),
                _ => (y, -x),
            }),
        }
    }
}

fn instruction_for_bearing(state: &State, distance: isize) -> Instr {
    match state.bearing {
        0 => Instr::East(distance),
        90 => Instr::North(distance),
        180 => Instr::West(distance),
        _ => Instr::South(distance),
    }
}

fn next_state_naive(state: State, instruction: &Instr) -> State {
    match instruction {
        Instr::North(dist) => state.move_y(*dist),
        Instr::South(dist) => state.move_y(-dist),
        Instr::East(dist) => state.move_x(*dist),
        Instr::West(dist) => state.move_x(-dist),
        Instr::Left(angle) => state.rotate(*angle),
        Instr::Right(angle) => state.rotate(-angle),
        Instr::Forward(dist) => {
            let instr = instruction_for_bearing(&state, *dist);
            next_state_naive(state, &instr)
        }
    }
}

fn next_state(state: State, instruction: &Instr) -> State {
    match instruction {
        Instr::North(dist) => state.move_waypoint_y(*dist),
        Instr::South(dist) => state.move_waypoint_y(-dist),
        Instr::East(dist) => state.move_waypoint_x(*dist),
        Instr::West(dist) => state.move_waypoint_x(-dist),
        Instr::Left(angle) => state.rotate_waypoint(*angle),
        Instr::Right(angle) => state.rotate_waypoint(-angle),
        Instr::Forward(times) => state.move_toward_waypoint(*times),
    }
}

fn part1(instructions: &[Instr]) -> usize {
    let result = instructions.iter().fold(State::new(), next_state_naive);
    (result.position.0.abs() + result.position.1.abs()) as usize
}

fn part2(instructions: &[Instr]) -> usize {
    let result = instructions
        .iter()
        .fold(State::with_waypoint((10, 1)), next_state);
    (result.position.0.abs() + result.position.1.abs()) as usize
}

fn main() -> Result<()> {
    let instructions = read_lines("input/day12.txt")?;
    let result = part1(&instructions);
    println!("part 1: {}", result);

    let result = part2(&instructions);
    println!("part 2: {}", result);
    Ok(())
}
