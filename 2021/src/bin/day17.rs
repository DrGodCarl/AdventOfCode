use anyhow::Result;
use itertools::Itertools;
use num::integer::Roots;
use utils::read_file;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
#[display("target area: x={x_min}..{x_max}, y={y_min}..{y_max}")]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn triangular_number(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn inverse_triangular_number(r: i32) -> i32 {
    ((2 * r) as f64).sqrt() as i32
}

fn part1(area: &TargetArea) -> i32 {
    triangular_number(area.y_min)
}

fn t_bound(v_y: i32, y_pos: i32) -> i32 {
    ((4 * v_y.pow(2) + 4 * v_y - 8 * y_pos + 1).sqrt() + 2 * v_y + 1) / 2
}

fn x_pos(v_x: i32, t: i32, cap: i32) -> i32 {
    if t > v_x {
        cap
    } else {
        (2 * v_x - t + 1) * t / 2
    }
}

fn y_pos(v_y: i32, t: i32) -> i32 {
    v_y * t - (t - 1) * (t) / 2
}

fn intersects(&(v_x, v_y): &(i32, i32), area: &TargetArea) -> bool {
    let t_low = t_bound(v_y, area.y_max);
    let t_cap = t_bound(v_y, area.y_min);
    let x_cap = triangular_number(v_x);
    (t_low..=t_cap)
        .filter(|&t| {
            let y_pos = y_pos(v_y, t);
            area.y_min <= y_pos && y_pos <= area.y_max
        })
        .any(|t| {
            let x_pos = x_pos(v_x, t, x_cap);
            area.x_min <= x_pos && x_pos <= area.x_max
        })
}

fn part2(area: &TargetArea) -> usize {
    let min_vx = inverse_triangular_number(area.x_min);
    let max_vx = area.x_max;
    let min_vy = area.y_min;
    let max_vy = -area.y_min - 1;
    (min_vx..=max_vx)
        .cartesian_product(min_vy..=max_vy)
        .filter(|velocity| intersects(velocity, area))
        .count()
}

fn main() -> Result<()> {
    let area = read_file("input/day17.txt")?;
    let result = part1(&area);
    println!("part 1: {}", result);
    let result = part2(&area);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let area = read_file("input/test/day17.txt")?;
    let result = part1(&area);
    assert_eq!(result, 45);

    let result = part2(&area);
    assert_eq!(result, 112);

    Ok(())
}
