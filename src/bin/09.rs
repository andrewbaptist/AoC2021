use Point::*;

#[derive(Clone, Copy, Debug)]
enum Point {
    Wall,
    NotBasin(u32),
    Basin(u32, usize), // val, basin_id
}

fn get_value(p: &Point) -> u32 {
    match p {
        Wall => 9,
        NotBasin(x) => *x,
        Basin(x, _) => *x,
    }
}

fn update_point(point: &mut Point, basin_id: usize) -> bool {
    if let NotBasin(val) = *point {
        *point = Basin(val, basin_id);
        return true;
    }
    false
}

fn main() {
    let lines = include_str!("../input/9.txt").lines();
    let mut lines: Vec<Vec<Point>> = lines
        .map(|line| {
            line.chars()
                .map(|c| {
                    let d = c.to_digit(10).unwrap();
                    if let 0..=8 = d {
                        NotBasin(d)
                    } else {
                        Wall
                    }
                })
                .collect()
        })
        .collect();
    // surround with Walls
    lines.push(vec![Wall; lines[0].len()]);
    lines.insert(0, vec![Wall; lines[0].len()]);
    for line in lines.iter_mut() {
        line.push(Wall);
        line.insert(0, Wall);
    }

    // new basin for each min
    let mut basin_id = 0;
    let mut sum = 0;
    (1..&lines.len() - 1).for_each(|i| {
        (1..lines[i].len() - 1).for_each(|j| {
            let val = get_value(&lines[i][j]);
            if val < get_value(&lines[i - 1][j])
                && val < get_value(&lines[i][j - 1])
                && val < get_value(&lines[i + 1][j])
                && val < get_value(&lines[i][j + 1])
            {
                sum += get_value(&lines[i][j]) + 1;
                update_point(&mut lines[i][j], basin_id);
                basin_id += 1;
            }
        });
    });
    println!("Part 1 {}", sum);

    // grow basins as much as possible
    let mut has_change = true;
    while has_change {
        has_change = false;
        (1..&lines.len() - 1).for_each(|i| {
            (1..lines[i].len() - 1).for_each(|j| {
                if let Basin(_, basin_id) = lines[i][j] {
                    has_change |= update_point(&mut lines[i][j - 1], basin_id);
                    has_change |= update_point(&mut lines[i][j + 1], basin_id);
                    has_change |= update_point(&mut lines[i - 1][j], basin_id);
                    has_change |= update_point(&mut lines[i + 1][j], basin_id);
                }
            });
        });
    }

    // count basin sizes
    let mut counts = vec![0; basin_id];
    (1..&lines.len() - 1).for_each(|i| {
        (1..lines[i].len() - 1).for_each(|j| {
            if let Basin(_, basin_id) = lines[i][j] {
                counts[basin_id] += 1;
            }
        });
    });
    counts.sort_unstable();
    counts.reverse();
    println!("Part 2 {}", counts[0] * counts[1] * counts[2]);
}
