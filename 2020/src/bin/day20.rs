#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{bail, Result};
use itertools::Itertools;
use regex::Regex;
use utils::{read_chunks, InputParseError};

trait MutTransformable {
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn rotate_180(&mut self);
    fn flip_x(&mut self);
    fn flip_y(&mut self);
}

trait Transformable {
    fn by_rotating_left(&self) -> Self;
    fn by_rotating_right(&self) -> Self;
    fn by_rotating_180(&self) -> Self;
    fn by_flipping_over_x(&self) -> Self;
    fn by_flipping_over_y(&self) -> Self;
}

// only for square strings
impl Transformable for String {
    fn by_rotating_left(&self) -> Self {
        let mut matrix = self
            .split('\n')
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in i..n - i - 1 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][n - 1 - i];
                matrix[j][n - 1 - i] = matrix[n - 1 - i][n - 1 - j];
                matrix[n - 1 - i][n - 1 - j] = matrix[n - 1 - j][i];
                matrix[n - 1 - j][i] = temp;
            }
        }
        matrix.iter().map(|cs| cs.iter().join("")).join("\n")
    }

    fn by_rotating_right(&self) -> Self {
        self.by_rotating_180().by_rotating_left()
    }

    fn by_rotating_180(&self) -> Self {
        self.by_rotating_left().by_rotating_left()
    }

    fn by_flipping_over_x(&self) -> Self {
        self.split('\n').rev().join("\n")
    }

    fn by_flipping_over_y(&self) -> Self {
        self.split('\n').map(|l| l.to_string().reverse()).join("\n")
    }
}

struct MapChunk {
    id: u64,
    body: String,
    top: String,
    right: String,
    bottom: String,
    left: String,
}

impl MapChunk {
    // part of a hack to get around the entry api
    fn new(id: u64) -> Self {
        MapChunk {
            id,
            body: "".to_string(),
            top: "".to_string(),
            right: "".to_string(),
            bottom: "".to_string(),
            left: "".to_string(),
        }
    }

    fn edges(&self) -> Vec<&String> {
        vec![&self.top, &self.bottom, &self.left, &self.right]
    }

    fn edge(&self, dir: &Dir) -> &String {
        match dir {
            Dir::Left => &self.left,
            Dir::Right => &self.right,
            Dir::Up => &self.top,
            Dir::Down => &self.bottom,
        }
    }

    #[allow(clippy::ptr_arg)]
    fn make_side_match(&mut self, side: &Dir, to_match: &String) {
        let matchable = vec![to_match.clone(), to_match.reverse()];
        while !matchable.contains(self.edge(side)) {
            self.rotate_left();
        }
        if self.edge(side) != to_match {
            match side {
                Dir::Left | Dir::Right => self.flip_x(),
                Dir::Up | Dir::Down => self.flip_y(),
            }
        }
    }
}

impl MutTransformable for MapChunk {
    fn rotate_left(&mut self) {
        let new_left = self.top.reverse();
        self.top = self.right.clone();
        self.right = self.bottom.reverse();
        self.bottom = self.left.clone();
        self.left = new_left;
        self.body = self.body.by_rotating_left();
    }

    fn rotate_right(&mut self) {
        self.rotate_180();
        self.rotate_left();
    }

    fn rotate_180(&mut self) {
        self.rotate_left();
        self.rotate_left();
    }

    fn flip_x(&mut self) {
        let new_top = self.bottom.clone();
        self.right = self.right.reverse();
        self.bottom = self.top.clone();
        self.left = self.left.reverse();
        self.top = new_top;
        self.body = self.body.by_flipping_over_x();
    }

    fn flip_y(&mut self) {
        let new_left = self.right.clone();
        self.top = self.top.reverse();
        self.right = self.left.clone();
        self.bottom = self.bottom.reverse();
        self.left = new_left;
        self.body = self.body.by_flipping_over_y();
    }
}

impl FromStr for MapChunk {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        }
        let lines: Vec<&str> = s.split('\n').collect();
        let id_line = lines.get(0).ok_or(InputParseError)?;
        let id = ID_RE
            .captures(id_line)
            .and_then(|c| c.get(1))
            .map(|i| i.as_str().parse().map_err(|_| InputParseError))
            .unwrap_or(Err(InputParseError))?;
        let chunk = &lines[1..];
        let top = chunk[0].to_string();
        let right = chunk
            .iter()
            .map(|l| l.chars().last())
            .collect::<Option<Vec<char>>>()
            .map(|l| l.iter().join(""))
            .ok_or(InputParseError)?;
        let bottom = chunk[chunk.len() - 1].to_string();
        let left = chunk
            .iter()
            .map(|l| l.chars().next())
            .collect::<Option<Vec<char>>>()
            .map(|l| l.iter().join(""))
            .ok_or(InputParseError)?;
        let body = chunk
            .iter()
            .skip(1)
            .take(chunk.len() - 2)
            .map(|l| l[1..l.len() - 1].to_string())
            .join("\n");
        Ok(MapChunk {
            id,
            body,
            top,
            right,
            bottom,
            left,
        })
    }
}

trait Reversable {
    fn reverse(&self) -> Self;
}

impl Reversable for String {
    fn reverse(&self) -> Self {
        self.chars().rev().collect()
    }
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

struct PuzzleMap {
    puzzle_pieces: HashMap<u64, MapChunk>,
    solution: HashMap<(isize, isize), u64>,
    all_edges: HashMap<String, Vec<u64>>,
}

impl PuzzleMap {
    fn new(pieces: Vec<MapChunk>) -> Self {
        let puzzle_pieces: HashMap<u64, MapChunk> = pieces.into_iter().map(|p| (p.id, p)).collect();
        let all_edges = puzzle_pieces
            .values()
            .flat_map(|c| {
                c.edges()
                    .iter()
                    .map(|&e| (e.clone(), c.id))
                    .collect::<Vec<_>>()
            })
            .sorted_by_key(|(e, _)| e.clone())
            .group_by(|(e, _)| e.clone())
            .into_iter()
            .map(|(key, group)| {
                (
                    key,
                    group
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|(_, id)| *id)
                        .collect::<Vec<u64>>(),
                )
            })
            .collect();
        PuzzleMap {
            puzzle_pieces,
            solution: HashMap::new(),
            all_edges,
        }
    }

    #[allow(clippy::ptr_arg)]
    fn get_matches_for_edge(&self, edge: &String) -> Vec<u64> {
        [edge, &edge.reverse()]
            .iter()
            .flat_map(|&e| self.all_edges.get(e).cloned().unwrap_or_default())
            .collect()
    }

    fn edges_with_matches(&self, chunk: &MapChunk) -> Vec<String> {
        // this is wrong now I think
        chunk
            .edges()
            .iter()
            .filter(|&&e| self.get_matches_for_edge(e).len() == 2)
            .map(|&s| s.clone())
            .collect()
    }

    fn find_top_left(&self) -> Result<u64> {
        let top_left = self
            .puzzle_pieces
            .values()
            .find(|&m| self.edges_with_matches(m).len() == 2)
            .map(|i| i.id);
        if top_left.is_none() {
            bail!("Puzzle has no corner pieces.");
        }
        Ok(top_left.unwrap())
    }

    fn find_neighbor(&self, chunk_id: u64, direction: Dir) -> Option<u64> {
        let chunk = self.puzzle_pieces.get(&chunk_id).unwrap();
        self.get_matches_for_edge(chunk.edge(&direction))
            .iter()
            .find(|&i| i != &chunk_id)
            .copied()
    }

    fn solve(&mut self) -> Result<()> {
        let top_left_id = self.find_top_left()?;
        self.solution.insert((0, 0), top_left_id);

        let chunk = self.puzzle_pieces.get(&top_left_id).unwrap();
        let bottom_right_edges = self.edges_with_matches(chunk);

        let entry = self
            .puzzle_pieces
            .entry(top_left_id)
            .or_insert_with(|| MapChunk::new(top_left_id));
        while !((bottom_right_edges.contains(&entry.right)
            || bottom_right_edges.contains(&entry.right.reverse()))
            && (bottom_right_edges.contains(&entry.bottom)
                || bottom_right_edges.contains(&entry.bottom.reverse())))
        {
            entry.rotate_left();
        }

        let mut first_in_row_id = top_left_id;
        let mut current_id = top_left_id;
        let mut x = 0;
        let mut y = 0;
        loop {
            let neighbor_id = self.find_neighbor(current_id, Dir::Right);
            match neighbor_id {
                Some(id) => {
                    let entry_right = self.puzzle_pieces.get(&current_id).unwrap().right.clone();
                    x += 1;
                    self.solution.insert((x, y), id);
                    let neighbor = self
                        .puzzle_pieces
                        .entry(id)
                        .or_insert_with(|| MapChunk::new(id));
                    neighbor.make_side_match(&Dir::Left, &entry_right);
                    current_id = id;
                }
                None => {
                    let neighbor_id = self.find_neighbor(first_in_row_id, Dir::Down);
                    let row_start_down = self
                        .puzzle_pieces
                        .get(&first_in_row_id)
                        .unwrap()
                        .bottom
                        .clone();
                    match neighbor_id {
                        Some(id) => {
                            y += 1;
                            x = 0;
                            self.solution.insert((x, y), id);
                            let neighbor = self
                                .puzzle_pieces
                                .entry(id)
                                .or_insert_with(|| MapChunk::new(id));
                            neighbor.make_side_match(&Dir::Up, &row_start_down);
                            first_in_row_id = id;
                            current_id = id;
                        }
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }

    fn max_coords(&self) -> (isize, isize) {
        self.solution
            .keys()
            .max_by_key(|(x, y)| x * y)
            .copied()
            .unwrap()
    }

    fn get_body_at_coords(&self, coords: &(isize, isize)) -> &String {
        let id = self.solution.get(coords).unwrap();
        &self.puzzle_pieces.get(id).unwrap().body
    }

    fn stitch_together_solution(&self) -> String {
        let (max_x, max_y) = self.max_coords();
        let example_body = self.get_body_at_coords(&(0, 0));
        let size = example_body.split('\n').count();
        let mut out = String::new();
        for y in 0..=max_y {
            for line_no in 0..size {
                for x in 0..=max_x {
                    let body = self.get_body_at_coords(&(x, y));
                    let line = body.split('\n').nth(line_no).unwrap();
                    out += line;
                }
                out += "\n";
            }
        }
        out.strip_suffix("\n").unwrap().to_string()
    }
}

#[allow(clippy::ptr_arg)]
fn get_char_at(string: &String, coord: (isize, isize)) -> Option<char> {
    if coord.0 < 0 || coord.1 < 0 {
        return None;
    }
    let coord = (coord.0 as usize, coord.1 as usize);
    string
        .split('\n')
        .nth(coord.1)
        .and_then(|l| l.chars().nth(coord.0))
}

fn part1(puzzle: &PuzzleMap) -> u64 {
    let (max_x, max_y) = puzzle.max_coords();
    [(0, 0), (0, max_y), (max_x, 0), (max_x, max_y)]
        .iter()
        .map(|k| puzzle.solution.get(k).unwrap())
        .product()
}

// start here (+0, +0)
// |
// v                 #
// #    ##    ##    ###
//  #  #  #  #  #  #
//
fn part2(puzzle: &PuzzleMap) -> usize {
    let image = puzzle.stitch_together_solution();
    let sea_monster_relative_positions: Vec<(isize, isize)> = vec![
        (0, 0),
        (1, 1),
        (4, 1),
        (5, 0),
        (6, 0),
        (7, 1),
        (10, 1),
        (11, 0),
        (12, 0),
        (13, 1),
        (16, 1),
        (17, 0),
        (18, 0),
        (18, -1),
        (19, 0),
    ];
    let size = image.split('\n').next().map(|l| l.len()).unwrap_or(0) as isize;
    let image_possibilities = vec![
        image.clone(),
        image.by_rotating_left(),
        image.by_rotating_180(),
        image.by_rotating_right(),
        image.by_flipping_over_y(),
        image.by_flipping_over_y().by_rotating_left(),
        image.by_flipping_over_y().by_rotating_180(),
        image.by_flipping_over_y().by_rotating_right(),
        image.by_flipping_over_x().by_rotating_left(),
        image.by_flipping_over_x().by_rotating_180(),
        image.by_flipping_over_x().by_rotating_right(),
    ];
    let removed_point_count = image_possibilities
        .iter()
        .map(|i| {
            (0..size)
                .cartesian_product(0..size)
                .filter(|(x, y)| {
                    sea_monster_relative_positions
                        .iter()
                        .all(|(delta_x, delta_y)| {
                            get_char_at(i, (x + delta_x, y + delta_y)) == Some('#')
                        })
                })
                .flat_map(|(x, y)| {
                    sea_monster_relative_positions
                        .iter()
                        .map(|(delta_x, delta_y)| (x + delta_x, y + delta_y))
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>()
        })
        .map(|s| s.len())
        .find(|l| l != &0)
        .unwrap_or(0);

    image.chars().filter(|c| c == &'#').count() - removed_point_count
}

fn main() -> Result<()> {
    let map_chunks = read_chunks("input/day20.txt")?;
    let mut puzzle = PuzzleMap::new(map_chunks);
    puzzle.solve()?;

    let result = part1(&puzzle);
    println!("part 1: {}", result);

    let result = part2(&puzzle);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let map_chunks: Vec<MapChunk> = read_chunks("input/test/day20.txt")?;
        let chunk = &map_chunks[0];
        let expected_body = "\
        #..#....\n\
        ...##..#\n\
        ###.#...\n\
        #.##.###\n\
        #...#.##\n\
        #.#.#..#\n\
        .#....#.\n\
        ##...#.#";
        assert_eq!(chunk.body, expected_body);
        assert_eq!(chunk.top, "..##.#..#.");
        assert_eq!(chunk.bottom, "..###..###");
        assert_eq!(chunk.left, ".#####..#.");
        assert_eq!(chunk.right, "...#.##..#");

        Ok(())
    }

    #[test]
    fn test_stitch() -> Result<()> {
        let map_chunks = read_chunks("input/test/day20.txt")?;
        let mut puzzle = PuzzleMap::new(map_chunks);
        puzzle.solve()?;
        let image = puzzle.stitch_together_solution();
        let expected: String = read_file("input/test/day20_res.txt")?;

        let image_possibilities = vec![
            image.clone(),
            image.by_rotating_left(),
            image.by_rotating_180(),
            image.by_rotating_right(),
            image.by_flipping_over_y(),
            image.by_flipping_over_y().by_rotating_left(),
            image.by_flipping_over_y().by_rotating_180(),
            image.by_flipping_over_y().by_rotating_right(),
            image.by_flipping_over_x().by_rotating_left(),
            image.by_flipping_over_x().by_rotating_180(),
            image.by_flipping_over_x().by_rotating_right(),
        ];
        if image_possibilities.contains(&expected) {
            Ok(())
        } else {
            bail!("Couldn't find a match")
        }
    }

    #[test]
    fn test_part1() -> Result<()> {
        let map_chunks = read_chunks("input/test/day20.txt")?;
        let mut puzzle = PuzzleMap::new(map_chunks);
        puzzle.solve()?;
        let result = part1(&puzzle);
        assert_eq!(result, 20899048083289);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let map_chunks = read_chunks("input/test/day20.txt")?;
        let mut puzzle = PuzzleMap::new(map_chunks);
        puzzle.solve()?;
        let result = part2(&puzzle);
        assert_eq!(result, 273);
        Ok(())
    }
}

#[cfg(test)]
mod transformable_tests {
    use super::*;

    #[test]
    fn test_rotating_string_left() {
        let input = "\
        AB\n\
        CD"
        .to_string();
        let expected = "\
        BD\n\
        AC"
        .to_string();
        let output = input.by_rotating_left();
        assert_eq!(output, expected);

        let input = "\
        ABC\n\
        DEF\n\
        GHI"
        .to_string();
        let expected = "\
        CFI\n\
        BEH\n\
        ADG"
        .to_string();
        let output = input.by_rotating_left();
        assert_eq!(output, expected);

        let input = "\
        ABCD\n\
        EFGH\n\
        IJKL\n\
        MNOP"
            .to_string();
        let expected = "\
        DHLP\n\
        CGKO\n\
        BFJN\n\
        AEIM"
            .to_string();
        let output = input.by_rotating_left();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_rotating_string_right() {
        let input = "\
        AB\n\
        CD"
        .to_string();
        let expected = "\
        CA\n\
        DB"
        .to_string();
        let output = input.by_rotating_right();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_rotating_string_180() {
        let input = "\
        AB\n\
        CD"
        .to_string();
        let expected = "\
        DC\n\
        BA"
        .to_string();
        let output = input.by_rotating_180();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_flipping_string_over_x() {
        let input = "\
        ABCD\n\
        EFGH\n\
        IJKL\n\
        MNOP"
            .to_string();
        let expected = "\
        MNOP\n\
        IJKL\n\
        EFGH\n\
        ABCD"
            .to_string();
        let output = input.by_flipping_over_x();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_flipping_string_over_y() {
        let input = "\
        ABCD\n\
        EFGH\n\
        IJKL\n\
        MNOP"
            .to_string();
        let expected = "\
        DCBA\n\
        HGFE\n\
        LKJI\n\
        PONM"
            .to_string();
        let output = input.by_flipping_over_y();
        assert_eq!(output, expected);
    }
}
