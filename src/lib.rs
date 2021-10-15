use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

type Map<K, V> = HashMap<K, V>;
type Set<T> = HashSet<T>;

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub cells: Map<Position, Cell>,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (l, _, w) = self.width();
        let (b, _, h) = self.height();
        let width = 80;
        let height = 50;
        // let left = if w > width * 2 {
        //     if l.abs() > r.abs() {
        //         l
        //     } else {
        //         r
        //     }
        // } else {
        //     l + w as i32/2
        // } - width as i32 / 2;
        let left = l + w as i32 / 2 - width as i32 / 2;
        // let bottom = if h > height * 2 {
        //     if b.abs() > t.abs() {
        //         b
        //     } else {
        //         t
        //     }
        // } else {
        //     b + h as i32/2
        // } - height as i32 / 2;
        let bottom = b + h as i32 / 2 - height as i32 / 2;

        self.write(left, bottom, width, height, None)?;
        writeln!(f, "({}, {})", left, bottom)?;
        Ok(())
    }
}

impl World {
    pub fn random(w: usize, h: usize, density: f64) -> Self {
        assert!(density >= 0.0 && density <= 1.0);
        let mut rng = thread_rng();
        let percent_chance = Uniform::from(0..100);
        let at_most = (density * 100.0) as usize;

        let mut grid = vec![];
        for _ in 0..h {
            let mut row = vec![];
            for _ in 0..w {
                row.push(if percent_chance.sample(&mut rng) <= at_most {
                    1
                } else {
                    0
                })
            }
            grid.push(row);
        }
        Self::grid(grid)
    }

    fn write(
        &self,
        left: i32,
        bottom: i32,
        width: usize,
        height: usize,
        f: Option<&mut Formatter>,
    ) -> FmtResult {
        let right = left + width as i32;
        let top = bottom + height as i32;

        let width = (right - left) as usize;
        let height = height as i32;
        match f {
            Some(f) => {
                // writeln!(f, "\x1b[2J\x1b[H+{}+", "-".repeat(width))?;
                writeln!(f, "\x1b[2J\x1b[H+{}+", "-".repeat(width))?;
                for y in 0..height {
                    write!(f, "|")?;
                    for x in left..right {
                        if self.is_alive(Position(x, top - y)) {
                            // write!(f, "\x1b[95m█\x1b[m\x1b[0m")
                            write!(f, "█")
                        } else {
                            write!(f, " ")
                        }?;
                    }
                    writeln!(f, "|")?;
                }
                writeln!(f, "+{}+", "-".repeat(width))?;
                println!("cells: {}", self.cells.len());
                Ok(())
            }
            None => {
                println!("\x1b[2J\x1b[H+{}+", "-".repeat(width));
                for y in 0..height {
                    print!("|");
                    for x in left..right {
                        if self.is_alive(Position(x, top - y)) {
                            // print!("\x1b[95m█\x1b[m\x1b[0m")
                            print!("█")
                        } else {
                            print!(" ")
                        }
                    }
                    println!("|");
                }
                println!("+{}+", "-".repeat(width));
                println!("cells: {}", self.cells.len());
                Ok(())
            }
        }
    }

    pub fn draw(&self, left: i32, bottom: i32, width: usize, height: usize) {
        self.write(left, bottom, width, height, None).unwrap();
    }

    pub fn draw_with_focus(&self, width: usize, height: usize) {
        let (l, _, w) = self.width();
        let (b, _, h) = self.height();
        // let left = if w > width * 2 {
        //     if l.abs() > r.abs() {
        //         l
        //     } else {
        //         r
        //     }
        // } else {
        //     l + w as i32/2
        // } - width as i32 / 2;
        let left = l + w as i32 / 2 - width as i32 / 2;
        // let bottom = if h > height * 2 {
        //     if b.abs() > t.abs() {
        //         b
        //     } else {
        //         t
        //     }
        // } else {
        //     b + h as i32/2
        // } - height as i32 / 2;
        let bottom = b + h as i32 / 2 - height as i32 / 2;

        self.write(left, bottom, width, height, None).unwrap();
    }

    pub fn grid(rows: impl IntoIterator<Item = impl IntoIterator<Item = usize>>) -> Self {
        let mut cells = vec![];
        // let mut height = 0;
        let grid = rows
            .into_iter()
            .map(|row| row.into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = grid.len();
        for (y, row) in grid.iter().enumerate() {
            for (x, alive) in row.iter().enumerate() {
                if *alive != 0 {
                    cells.push(Position(x as i32, height as i32 - y as i32));
                }
            }
        }
        Self::from(cells)
    }

    pub fn width(&self) -> (i32, i32, usize) {
        let mut left_most = None;
        let mut right_most = None;
        for Position(x, _) in self.cells.keys() {
            right_most = Some(match right_most {
                Some(n) => max(n, *x),
                None => *x,
            });
            left_most = Some(match left_most {
                Some(n) => min(n, *x),
                None => *x,
            });
        }

        let left_most = left_most.unwrap_or(0);
        let right_most = right_most.unwrap_or(0);
        (left_most, right_most, (right_most - left_most) as usize + 1)
    }

    pub fn height(&self) -> (i32, i32, usize) {
        let mut top_most = None;
        let mut bottom_most = None;
        for Position(_, y) in self.cells.keys() {
            top_most = Some(match top_most {
                Some(n) => max(n, *y),
                None => *y,
            });
            bottom_most = Some(match bottom_most {
                Some(n) => min(n, *y),
                None => *y,
            });
        }
        let bottom_most = bottom_most.unwrap_or(0);
        let top_most = top_most.unwrap_or(0);
        (bottom_most, top_most, (top_most - bottom_most) as usize + 1)
    }

    pub fn is_alive(&self, pos: Position) -> bool {
        self.cells.contains_key(&pos)
    }

    fn count_neighbors(&self, pos: Position) -> u8 {
        let mut result = 0;
        for x in Cell::from(pos) {
            result += self.is_alive(x) as u8
        }
        result
    }
}

impl FromStr for World {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = vec![];
        let height = s.lines().count();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'O' || c == 'o' || c == '0' {
                    cells.push(Position(x as i32, height as i32 - y as i32));
                }
            }
        }
        Ok(Self::from(cells))
    }
}

impl<T> From<T> for World
where
    T: IntoIterator<Item = Position>,
{
    fn from(positions: T) -> Self {
        let mut cells = Map::new();
        for pos in positions {
            cells.insert(pos, Cell::from(pos));
        }
        World { cells }
    }
}

impl Iterator for World {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let mut considered = Set::new();
        let old_self = self.clone();
        for cell in self.cells.clone().values() {
            let adjacent_count = old_self.count_neighbors(cell.pos);
            if adjacent_count < 2 || adjacent_count > 3 {
                self.cells.remove(&cell.pos);
            }

            for neighbor_pos in *cell {
                if considered.contains(&neighbor_pos) {
                    continue;
                } else {
                    considered.insert(neighbor_pos);
                }

                let adjacent_count = old_self.count_neighbors(neighbor_pos);
                let is_alive = old_self.is_alive(neighbor_pos);
                if is_alive && (adjacent_count < 2 || adjacent_count > 3) {
                    self.cells.remove(&neighbor_pos);
                } else if !is_alive && adjacent_count == 3 {
                    self.cells.insert(neighbor_pos, Cell::from(neighbor_pos));
                }
            }
        }
        if &old_self == self {
            None
        } else {
            Some(self.clone())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(pub i32, pub i32);

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    pub pos: Position,
    neighbor: u8,
}

impl From<Position> for Cell {
    fn from(pos: Position) -> Self {
        Cell { pos, neighbor: 0 }
    }
}

impl Iterator for Cell {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        match self.neighbor {
            0..=8 => {
                // Skip (0, 0) neighbor
                if self.neighbor == 4 {
                    self.neighbor += 1;
                }

                let x = (self.neighbor % 3) as i32 - 1;
                let y = (self.neighbor / 3) as i32 - 1;
                self.neighbor += 1;

                Some(Position(self.pos.0 + x as i32, self.pos.1 + y as i32))
            }
            _ => None,
        }
    }
}
