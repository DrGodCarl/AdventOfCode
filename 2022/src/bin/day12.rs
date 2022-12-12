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

fn run_2(
    map: &Grid<i16, MapPoint>,
    start: MapPoint,
    end: &dyn Fn(MapPoint) -> bool,
    test_valid: &dyn Fn(u32, u32) -> bool,
) -> Option<u32> {
    let start = map
        .iter()
        .find(|(_, p)| *p == &start)
        .map(|(p, _)| p)
        .unwrap();
    let path = bfs(
        start,
        |p| {
            let height = map.get(p).map(|h| h.height()).unwrap_or(u32::MIN);
            let res = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|(x, y)| (x + p.0, y + p.1))
                .filter(|n| {
                    map.get(n)
                        .map(|nh| test_valid(nh.height(), height))
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>();
            res
        },
        |p| end(map.get(p).copied().unwrap_or(MapPoint::Point(' '))),
    );
    path.map(|p| p.len() as u32 - 1)
}

fn part1(map: &Grid<i16, MapPoint>) -> Option<u32> {
    run_2(map, MapPoint::Start, &|v| v == MapPoint::End, &|a, b| {
        a <= b + 1
    })
}

fn part2(map: &Grid<i16, MapPoint>) -> Option<u32> {
    run_2(
        map,
        MapPoint::End,
        &|v| v == MapPoint::Start || v == MapPoint::Point('a'),
        &|a, b| a >= b - 1,
    )
}

fn main() -> Result<()> {
    let map = read_grid("input/day12.txt")?;
    let result = part1(&map);
    println!("part 1: {:?}", result);
    let result = part2(&map);
    println!("part 2: {:?}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let map = read_grid("input/test/day12.txt")?;
    let result = part1(&map);
    assert_eq!(result, Some(31));

    let result = part2(&map);
    assert_eq!(result, Some(29));

    Ok(())
}
