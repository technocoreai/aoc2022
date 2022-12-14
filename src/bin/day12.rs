use std::collections::BTreeSet;
use utils::{aoc_main, Matrix, Point2D};

#[derive(Debug)]
struct Heightmap {
    starting_pos: Point2D,
    target_pos: Point2D,
    elevations: Matrix<i64, 2>,
}

fn parse(input: &str) -> Heightmap {
    let mut elevations = Matrix::empty();
    let mut starting_pos = [0, 0];
    let mut target_pos = [0, 0];

    for (row, line) in input.lines().enumerate() {
        let elevation_row: Vec<i64> = line
            .chars()
            .enumerate()
            .map(|(col, c)| match c {
                'S' => {
                    starting_pos = [col, row];
                    0
                }
                'E' => {
                    target_pos = [col, row];
                    25
                }
                'a'..='z' => (c as i64) - ('a' as i64),
                _ => panic!("Invalid character: {}", c),
            })
            .collect();
        elevations.add_row(elevation_row.as_slice());
    }

    Heightmap {
        starting_pos,
        target_pos,
        elevations,
    }
}

fn pop(visit_queue: &mut BTreeSet<(u64, Point2D)>) -> Option<(u64, Point2D)> {
    let result = visit_queue.iter().next().copied();
    if let Some(elem) = result {
        visit_queue.remove(&elem);
    }
    result
}

fn compute_movement_costs(
    heightmap: &Matrix<i64, 2>,
    starting_pos: Point2D,
    can_move: fn(i64, i64) -> bool,
) -> Matrix<u64, 2> {
    let mut best_costs: Matrix<u64, 2> =
        Matrix::new(u64::MAX, [heightmap.width(), heightmap.height()]);
    let mut visit_queue: BTreeSet<(u64, Point2D)> = BTreeSet::new();

    best_costs[starting_pos] = 0;
    visit_queue.insert((0, starting_pos));

    while let Some((cost, coords)) = pop(&mut visit_queue) {
        let height = heightmap[coords];
        println!("Checking {coords:?}, height {height}, current cost {cost}");
        for neighbour in heightmap.neighbours(coords) {
            let nheight = heightmap[neighbour];
            if !can_move(height, nheight) {
                continue;
            }

            let new_best_cost = cost + 1;
            let current_best_cost = best_costs[neighbour];
            if current_best_cost <= new_best_cost {
                continue;
            }

            visit_queue.remove(&(current_best_cost, neighbour));

            println!(" - Updating cost for {neighbour:?} to {new_best_cost}");
            best_costs[neighbour] = new_best_cost;
            visit_queue.insert((new_best_cost, neighbour));
        }
    }

    best_costs
}

fn part1(input: &str) -> u64 {
    let heightmap = parse(input);
    let costs = compute_movement_costs(
        &heightmap.elevations,
        heightmap.starting_pos,
        |elev, nelev| nelev <= elev + 1,
    );
    costs[heightmap.target_pos]
}

fn part2(input: &str) -> u64 {
    let heightmap = parse(input);
    let costs = compute_movement_costs(
        &heightmap.elevations,
        heightmap.target_pos,
        |elev, nelev| elev <= nelev + 1,
    );
    heightmap
        .elevations
        .enumerate()
        .filter_map(|(coords, elev)| {
            if *elev == 0 {
                Some(costs[coords])
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 29);
    }
}
