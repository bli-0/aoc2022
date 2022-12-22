use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/22");

#[time_run2("22")]
fn main() {
    monkey_map(INPUT)
}

fn monkey_map(i: &str) -> (String, String) {
    let (map, instruction_str) = i.split_once("\n\n").unwrap();
    let mut grid = Grid::from_str(map);

    let mut instructions: Vec<Instruction> = vec![];
    let mut last = 0;
    for (index, lr) in instruction_str.match_indices(|c| c == 'L' || c == 'R') {
        if last != index {
            instructions.push(Instruction::Move(
                instruction_str[last..index].parse::<usize>().unwrap(),
            ));
        }
        match lr {
            "L" => instructions.push(Instruction::RotateLeft),
            "R" => instructions.push(Instruction::RotateRight),
            _ => panic!("unexpected {}", lr),
        }
        last = index + lr.len();
    }
    if last < instruction_str.len() {
        instructions.push(Instruction::Move(
            instruction_str[last..].parse::<usize>().unwrap(),
        ));
    }
    grid.set_starting_point();
    for i in instructions {
        grid.do_instruction(i);
    }

    (grid.get_part1().to_string(), "".to_string())
}

#[derive(Debug, Clone)]
struct Grid {
    // Y, X
    map: Vec<Vec<Square>>,
    my_location: (usize, usize),
    my_facing: Facing,
}

impl Grid {
    fn from_str(i: &str) -> Self {
        let mut map: Vec<Vec<Square>> = vec![];
        for line in i.lines() {
            let mut things: Vec<Square> = vec![];
            for c in line.chars() {
                match c {
                    ' ' => things.push(Square::None),
                    '#' => things.push(Square::Wall),
                    '.' => things.push(Square::Open),
                    _ => panic!("unexpected char {}", c),
                }
            }
            map.push(things);
        }
        // Pad out the grid.
        let max_x = map.iter().map(|x| x.len()).max().unwrap();
        for x in map.iter_mut() {
            if x.len() < max_x {
                for _ in 0..max_x - x.len() {
                    x.push(Square::None)
                }
            }
        }

        Self {
            map,
            my_location: (0, 0),
            my_facing: Facing::Right,
        }
    }

    fn set_starting_point(&mut self) {
        let mut starting_x = 0;
        for (i, sq) in self.map[0].iter().enumerate() {
            if sq == &Square::Open {
                starting_x = i;
                break;
            }
        }

        self.map[0][starting_x] = Square::Me(Facing::Right);
        self.my_location = (starting_x, 0);
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Move(num_move) => match self.my_facing {
                Facing::Left => {
                    let mut x_range: Vec<usize> = (self.my_location.0..self.map[0].len()).collect();
                    x_range.extend::<Vec<usize>>((0..self.my_location.0).collect());

                    let mut moved_squares = 0;
                    for x in x_range.iter().rev() {
                        match self.map[self.my_location.1][*x] {
                            Square::Me(_) => {
                                self.print();
                                panic!("unexpected me!")
                            }
                            // Move self.
                            Square::Path(_) | Square::Open => {
                                self.map[self.my_location.1][self.my_location.0] =
                                    Square::Path(Facing::Left);
                                self.map[self.my_location.1][*x] = Square::Me(Facing::Left);
                                self.my_location = (*x, self.my_location.1);
                                moved_squares += 1;
                                if moved_squares == num_move {
                                    break;
                                }
                            }
                            // Stop
                            Square::Wall => {
                                break;
                            }
                            Square::None => continue,
                        }
                    }
                }
                Facing::Right => {
                    let mut x_range: Vec<usize> =
                        (self.my_location.0 + 1..self.map[0].len()).collect();
                    x_range.extend::<Vec<usize>>((0..=self.my_location.0).collect());

                    let mut moved_squares = 0;
                    for x in x_range.iter() {
                        match self.map[self.my_location.1][*x] {
                            Square::Me(_) => {
                                self.print();
                                panic!("unexpected me!")
                            }
                            // Move self.
                            Square::Path(_) | Square::Open => {
                                self.map[self.my_location.1][self.my_location.0] =
                                    Square::Path(Facing::Right);
                                self.map[self.my_location.1][*x] = Square::Me(Facing::Right);
                                self.my_location = (*x, self.my_location.1);
                                moved_squares += 1;
                                if moved_squares == num_move {
                                    break;
                                }
                            }
                            // Stop
                            Square::Wall => {
                                break;
                            }
                            Square::None => continue,
                        }
                    }
                }
                Facing::Up => {
                    let mut y_range: Vec<usize> = (self.my_location.1..self.map.len()).collect();
                    y_range.extend::<Vec<usize>>((0..self.my_location.1).collect());

                    let mut moved_squares = 0;
                    for y in y_range.iter().rev() {
                        match self.map[*y][self.my_location.0] {
                            Square::Me(_) => {
                                self.print();
                                panic!("unexpected me!")
                            }
                            // Move self.
                            Square::Path(_) | Square::Open => {
                                self.map[self.my_location.1][self.my_location.0] =
                                    Square::Path(Facing::Up);
                                self.map[*y][self.my_location.0] = Square::Me(Facing::Up);
                                self.my_location = (self.my_location.0, *y);
                                moved_squares += 1;
                                if moved_squares == num_move {
                                    break;
                                }
                            }
                            // Stop
                            Square::Wall => {
                                break;
                            }
                            Square::None => continue,
                        }
                    }
                }
                Facing::Down => {
                    let mut y_range: Vec<usize> =
                        (self.my_location.1 + 1..self.map.len()).collect();
                    y_range.extend::<Vec<usize>>((0..=self.my_location.1).collect());

                    let mut moved_squares = 0;
                    for y in y_range.iter() {
                        match self.map[*y][self.my_location.0] {
                            Square::Me(_) => {
                                self.print();
                                panic!("unexpected me!")
                            }
                            // Move self.
                            Square::Path(_) | Square::Open => {
                                self.map[self.my_location.1][self.my_location.0] =
                                    Square::Path(Facing::Down);
                                self.map[*y][self.my_location.0] = Square::Me(Facing::Down);
                                self.my_location = (self.my_location.0, *y);
                                moved_squares += 1;
                                if moved_squares == num_move {
                                    break;
                                }
                            }
                            // Stop
                            Square::Wall => {
                                break;
                            }
                            Square::None => continue,
                        }
                    }
                }
            },
            Instruction::RotateRight => match self.my_facing {
                Facing::Left => {
                    self.my_facing = Facing::Up;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Up);
                }
                Facing::Right => {
                    self.my_facing = Facing::Down;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Down);
                }
                Facing::Up => {
                    self.my_facing = Facing::Right;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Right);
                }
                Facing::Down => {
                    self.my_facing = Facing::Left;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Left);
                }
            },
            Instruction::RotateLeft => match self.my_facing {
                Facing::Left => {
                    self.my_facing = Facing::Down;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Down);
                }
                Facing::Right => {
                    self.my_facing = Facing::Up;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Up);
                }
                Facing::Up => {
                    self.my_facing = Facing::Left;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Left);
                }
                Facing::Down => {
                    self.my_facing = Facing::Right;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Right);
                }
            },
        }
    }

    fn get_part1(&self) -> u64 {
        let facing_score = match self.my_facing {
            Facing::Left => 2,
            Facing::Right => 0,
            Facing::Up => 3,
            Facing::Down => 1,
        };
        (self.my_location.1 + 1) as u64 * 1000 + (self.my_location.0 + 1) as u64 * 4 + facing_score
    }

    #[allow(unused)]
    fn print(&self) {
        for y in (0..self.map.len()) {
            eprint!("{:0>5} ", y);
            for x in (0..self.map[y].len()) {
                match self.map[y][x] {
                    Square::Me(direction) => match direction {
                        Facing::Left => eprint!("\x1B[0;31m<\x1b[0m"),
                        Facing::Right => eprint!("\x1B[0;31m>\x1b[0m"),
                        Facing::Up => eprint!("\x1B[0;31m^\x1b[0m"),
                        Facing::Down => eprint!("\x1B[0;31mv\x1b[0m"),
                    },
                    Square::Path(direction) => match direction {
                        Facing::Left => eprint!("<"),
                        Facing::Right => eprint!(">"),
                        Facing::Up => eprint!("^"),
                        Facing::Down => eprint!("v"),
                    },
                    Square::Open => eprint!("."),
                    Square::Wall => eprint!("#"),
                    Square::None => eprint!(" "),
                }
            }
            eprintln!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Me(Facing),
    Path(Facing),
    Open,
    Wall,
    None,
}

impl Default for Square {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Move(usize),
    RotateRight,
    RotateLeft,
}
