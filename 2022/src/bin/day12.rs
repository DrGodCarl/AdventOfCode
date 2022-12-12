use anyhow::Result;
use parse_display::FromStr;
use pathfinding::prelude::bfs;
use utils::{read_grid, Grid};

#[derive(FromStr, PartialEq, Eq, Debug, Clone, Copy)]
enum MapPoint {
    #[display("S")]
    Start,
    #[display("E")]
    End,
    #[display("{0}")]
    Point(char),
}

impl MapPoint {
    fn height(&self) -> u32 {
        match self {
            MapPoint::Start => 'a' as u32,
            MapPoint::End => 'z' as u32,
            MapPoint::Point(c) => *c as u32,
        }
    }
}

fn run_from(map: &Grid<i16, MapPoint>, start: &(i16, i16)) -> u32 {
    let path = bfs(
        start,
        |p| {
            let height = map.get(p).map(|h| h.height()).unwrap_or(u32::MIN);
            let res = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|(x, y)| (x + p.0, y + p.1))
                .filter(|n| map.get(n).map(|nh| nh.height()).unwrap_or(u32::MAX) <= height + 1)
                .collect::<Vec<_>>();
            res
        },
        |p| map.get(p) == Some(&MapPoint::End),
    );
    path.map(|p| p.len() as u32 - 1).unwrap_or(u32::MAX)
}

fn run(map: &Grid<i16, MapPoint>, test_starts: &[(i16, i16)]) -> u32 {
    test_starts
        .iter()
        .map(|start| run_from(map, start))
        .min()
        .unwrap_or(u32::MAX)
}

fn run_with_test(map: &Grid<i16, MapPoint>, test: &dyn Fn(MapPoint) -> bool) -> u32 {
    let starts = map
        .iter()
        .filter(|(_, &v)| test(v))
        .map(|(p, _)| p.clone())
        .collect::<Vec<_>>();
    run(map, &starts)
}

fn part1(map: &Grid<i16, MapPoint>) -> u32 {
    run_with_test(map, &|v| v == MapPoint::Start)
}

fn part2(map: &Grid<i16, MapPoint>) -> u32 {
    run_with_test(map, &|v| v == MapPoint::Start || v == MapPoint::Point('a'))
}

fn main() -> Result<()> {
    let map = read_grid("input/day12.txt")?;
    let result = part1(&map);
    println!("part 1: {}", result);
    let result = part2(&map);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let map = read_grid("input/test/day12.txt")?;
    let result = part1(&map);
    assert_eq!(result, 31);

    let result = part2(&map);
    assert_eq!(result, 29);

    Ok(())
}
