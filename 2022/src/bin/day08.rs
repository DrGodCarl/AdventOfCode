use std::{collections::HashMap, iter, ops::Not};

use anyhow::Result;
use itertools::Itertools;
use utils::read_grid;

#[derive(PartialEq, Clone, Copy, Debug)]
enum TreeComparison {
    AsTall,
    Shorter,
    NoTree,
}

fn directional_iters_from((x, y): &(u32, u32)) -> Vec<Box<dyn Iterator<Item = (u32, u32)> + '_>> {
    vec![
        Box::new((0..*y).rev().map(|y2| (*x, y2))), // north
        Box::new((y + 1..).map(|y2| (*x, y2))),     // south
        Box::new((x + 1..).map(|x2| (x2, *y))),     // east
        Box::new((0..*x).rev().map(|x2| (x2, *y))), // west
    ]
}

fn part1(forest: &HashMap<(u32, u32), u8>) -> usize {
    forest
        .iter()
        .filter_map(|(p, h)| {
            directional_iters_from(p)
                .into_iter()
                .filter_map(|points| {
                    points
                        .take_while(|p2| forest.contains_key(p2))
                        .filter_map(|p2| forest.get(&p2))
                        .any(|&h2| h2 >= *h)
                        // .filter can't consume `points` so this maps and filters out Nones.
                        .then_some(())
                })
                .count()
                .eq(&4)
                .not()
                .then_some(())
        })
        .count()
}

fn part2(forest: &HashMap<(u32, u32), u8>) -> usize {
    forest
        .iter()
        .map(|(p, h)| {
            directional_iters_from(p)
                .into_iter()
                .map(|points| {
                    // At the bottom of this chain we use a window of two and we need to know
                    // what the previous value was. To facilitate that, we add a None at the
                    // beginning of the chain.
                    iter::once(None)
                        .chain(
                            points
                                .map(|p2| {
                                    forest
                                        .get(&p2)
                                        .map(|&h2| {
                                            if h2 >= *h {
                                                TreeComparison::AsTall
                                            } else {
                                                TreeComparison::Shorter
                                            }
                                        })
                                        .unwrap_or(TreeComparison::NoTree)
                                })
                                .map(Some),
                        )
                        .tuple_windows()
                        .take_while(|(prev_comp, comp)| {
                            // If the current comparison is NoTree, we're done.
                            *comp != Some(TreeComparison::NoTree)
                                // If the previous comparison was AsTall as the current tree,
                                // we can't see trees beyond it so we're done. We still need to
                                // count it as a visible tree, hence the windowing.
                                && *prev_comp != Some(TreeComparison::AsTall)
                        })
                        .count()
                })
                .product()
        })
        .max()
        .unwrap_or(0)
}

fn main() -> Result<()> {
    let forest = read_grid("input/day08.txt")?;
    let result = part1(&forest);
    println!("part 1: {}", result);
    let result = part2(&forest);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let forest = read_grid("input/test/day08.txt")?;
    let result = part1(&forest);
    assert_eq!(result, 21);

    let result = part2(&forest);
    assert_eq!(result, 8);

    Ok(())
}
