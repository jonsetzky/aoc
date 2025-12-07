use std::{collections::HashMap, fs::read_to_string};

const MAX_ADJACENT: isize = 4;

fn count_accessable_rolls(grid: &str) -> usize {
    let w = grid.lines().next().unwrap().len();
    let h = grid.lines().count();
    let grid_no_newlines = grid.replace("\n", "");

    let total_rolls = grid_no_newlines.chars().filter(|c| *c == '@').count();
    let mut rolls: HashMap<(usize, usize), isize> = HashMap::new();

    println!("Grid size: {} x {}, {} rolls", w, h, total_rolls);

    let coords = |i: usize| -> (usize, usize) { (i % w, i / w) };

    grid_no_newlines
        .char_indices()
        .filter(|(_, c)| *c == '@')
        .for_each(|(i, _)| {
            let (x, y) = coords(i);
            let mut count = 8isize - MAX_ADJACENT;
            if x == 0 || x == w - 1 {
                count -= 3;
                if y == 0 || y == h - 1 {
                    count -= 2;
                }
            } else if y == 0 || y == h - 1 {
                count -= 3;
            }
            rolls.insert((x, y), count);
        });

    grid_no_newlines
        .char_indices()
        .filter(|(_, c)| *c == '.')
        .for_each(|(i, _)| {
            let (x, y) = coords(i);

            (-1..=1).for_each(|dy| {
                (-1..=1).for_each(|dx| {
                    if dx == 0 && dy == 0 {
                        return;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0 || nx >= w as isize || ny < 0 || ny >= h as isize {
                        return;
                    }
                    let roll = rolls.get(&(nx as usize, ny as usize));

                    if roll.is_some() {
                        rolls.insert((nx as usize, ny as usize), roll.unwrap() - 1);
                    }
                });
            });
        });

    // for j in 0..h {
    //     for i in 0..w {
    //         print!(
    //             "{}",
    //             rolls
    //                 .get(&(i, j))
    //                 .map(|x| x.to_string().chars().take(1).next().unwrap())
    //                 .unwrap_or('.')
    //         );
    //     }
    //     println!()
    // }
    rolls.into_iter().filter(|(_, count)| *count < 0).count() as usize
}

fn main() {
    let input_string = read_to_string("src/bin/04/input.txt").expect("Failed to read file");

    let start = std::time::Instant::now();

    let count = count_accessable_rolls(&input_string);

    let duration = start.elapsed();

    println!("Result: {}", count);
    println!("Time elapsed: {:?}", duration);
}

#[cfg(test)]
mod day04_tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        r##"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."##,
        13_usize
    )]
    fn it_works(grid: &'static str, expected: usize) {
        assert_eq!(expected, count_accessable_rolls(grid));
    }
}
