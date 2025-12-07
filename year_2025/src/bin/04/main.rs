use std::{cmp::min, fs::read_to_string};

fn count_accessable_rolls(grid: &str) -> usize {
    12
}

fn main() {
    let input_string = read_to_string("src/bin/04/input.txt").expect("Failed to read file");

    let start = std::time::Instant::now();

    // todo

    let duration = start.elapsed();

    println!("Result todo");
    println!("Time elapsed: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
",
        13
    )]
    fn it_works(grid: &str, expected: usize) {
        assert_eq!(expected, count_accessable_rolls(grid));
    }
}
