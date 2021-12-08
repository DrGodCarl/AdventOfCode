#[macro_use]
extern crate bitflags;

use anyhow::Result;
use utils::read_lines;

bitflags! {
    struct Segments: u8 {
        const TOP = 0b00000001;
        const TOP_LEFT = 0b00000010;
        const TOP_RIGHT = 0b00000100;
        const MIDDLE = 0b00001000;
        const BOTTOM_LEFT = 0b00010000;
        const BOTTOM_RIGHT = 0b00100000;
        const BOTTOM = 0b01000000;
        const NONE = 0b0;
    }
}

impl Segments {
    fn segment_from_char(c: char) -> Segments {
        match c {
            'a' => Segments::TOP,
            'b' => Segments::TOP_LEFT,
            'c' => Segments::TOP_RIGHT,
            'd' => Segments::MIDDLE,
            'e' => Segments::BOTTOM_LEFT,
            'f' => Segments::BOTTOM_RIGHT,
            'g' => Segments::BOTTOM,
            _ => Segments::NONE,
        }
    }

    fn segments_from_str(s: &str) -> Segments {
        s.chars()
            .map(Segments::segment_from_char)
            .fold(Segments::NONE, |acc, seg| acc | seg)
    }
}

impl Default for Segments {
    fn default() -> Self {
        Segments::NONE
    }
}

struct InputData {
    signal_pattern: Vec<Segments>,
    output: Vec<Segments>,
    numbers: [Segments; 10],
}

impl InputData {
    fn new(row: &str) -> Self {
        let input_output: Vec<_> = row.split(" | ").collect();
        let (pattern, out) = (input_output[0], input_output[1]);
        let signal_pattern = pattern
            .split(' ')
            .map(Segments::segments_from_str)
            .collect();
        let output = out.split(' ').map(Segments::segments_from_str).collect();
        Self {
            signal_pattern,
            output,
            numbers: Default::default(),
        }
    }

    fn identify_easy(&mut self) {
        for &pattern in self.signal_pattern.iter() {
            // 1, 4, 7, and 8 all unique counts of segments
            match pattern.bits.count_ones() {
                2 => self.numbers[1] = pattern,
                4 => self.numbers[4] = pattern,
                3 => self.numbers[7] = pattern,
                7 => self.numbers[8] = pattern,
                _ => (),
            }
        }
    }

    fn identify_6_segment(&mut self) {
        for &pattern in self.signal_pattern.iter() {
            if pattern.bits.count_ones() == 6 {
                // a 4 overlays completely -> 9
                if pattern & self.numbers[4] == self.numbers[4] {
                    self.numbers[9] = pattern;
                // a 1 overlays completely -> 0
                } else if pattern & self.numbers[1] == self.numbers[1] {
                    self.numbers[0] = pattern;
                // otherwise -> 6
                } else {
                    self.numbers[6] = pattern;
                }
            }
        }
    }

    fn identify_5_segment(&mut self) {
        for &pattern in self.signal_pattern.iter() {
            if pattern.bits.count_ones() == 5 {
                // a 1 overlays completely -> 3
                if pattern & self.numbers[1] == self.numbers[1] {
                    self.numbers[3] = pattern;
                // a 4 matches 3 segments -> 5
                } else if (pattern & self.numbers[4]).bits.count_ones() == 3 {
                    self.numbers[5] = pattern;
                // otherwise (4 matchs 2 segments) -> 2
                } else {
                    self.numbers[2] = pattern;
                }
            }
        }
    }

    fn solve(&mut self) {
        self.identify_easy(); // 1, 4, 7, 8
        self.identify_6_segment(); // 0, 6, 0
        self.identify_5_segment(); // 2, 3, 5
    }

    fn output_values(&self) -> usize {
        self.output
            .iter()
            .filter_map(|o| self.numbers.iter().position(|n| n == o))
            .fold(0, |acc, n| n + 10 * acc)
    }
}

fn part1(rows: &[String]) -> usize {
    rows.iter()
        .filter_map(|s| s.split(" | ").last())
        .flat_map(|s| s.split(' '))
        .filter(|&s| [2usize, 4, 3, 7].contains(&s.len()))
        .count()
}

fn part2(rows: &[String]) -> usize {
    let mut inputs: Vec<InputData> = rows.iter().map(|s| InputData::new(s)).collect();
    inputs.iter_mut().for_each(|i| i.solve());
    inputs.iter().map(InputData::output_values).sum()
}

fn main() -> Result<()> {
    let rows = read_lines("input/day08.txt")?;
    let result = part1(&rows);
    println!("part 1: {}", result);
    let result = part2(&rows);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let rows = read_lines("input/test/day08.txt")?;
        let result = part1(&rows);
        assert_eq!(result, 26);
        let result = part2(&rows);
        assert_eq!(result, 61229);

        Ok(())
    }

    #[test]
    fn test_solving() -> Result<()> {
        let rows = read_lines("input/test/day08_small.txt")?;
        let result = part2(&rows);
        assert_eq!(result, 5353);
        Ok(())
    }
}
