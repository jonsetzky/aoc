use std::fs::read_to_string;

/* Rotates the dial based on direction and value. Returns the new dial position and the number of times the dial passed zero. */
fn rotate_dial(dial: i32, value: i32) -> (i32, i32) {
    let mut rounds = 0;
    let was_at_zero = dial == 0;

    if dial < 0 || dial > 99 {
        panic!("Dial out of bounds: {}", dial);
    }

    let mut new_dial = dial + value;

    if new_dial < 0 {
        rounds = (((-new_dial) as f64) / 100.0).ceil() as i32;
        new_dial += 100 * rounds;
        if was_at_zero {
            rounds -= 1;
        }
        if new_dial == 0 {
            rounds += 1;
        }
    } else if new_dial > 99 {
        rounds = ((new_dial - 99) as f64 / 100.0).ceil() as i32;
        new_dial -= 100 * rounds;
    } else if new_dial == 0 && value != 0 {
        rounds += 1;
    }

    (new_dial, rounds)
}

fn main() {
    let input_string = read_to_string("src/bin/01/input.txt").expect("Failed to read file");

    let start = std::time::Instant::now();

    let mut dial = 50;
    let mut zero_count = 0;
    let mut zero_count_2 = 0;

    for line in input_string.lines() {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let mut value: i32 = chars.collect::<String>().parse().unwrap();

        match dir {
            'L' => value = -value,
            'R' => (),
            _ => panic!("Invalid direction: {}", dir),
        }

        let (new_dial, passes) = rotate_dial(dial, value);
        zero_count_2 += passes;

        if new_dial == 0 {
            zero_count += 1;
        }

        dial = new_dial
    }

    let duration = start.elapsed();

    println!("Final dial position: {}", dial);
    println!("Number of times dial stopped at 0: {}", zero_count);
    println!("Number of times dial was at 0: {}", zero_count_2);
    println!("Time elapsed: {:?}", duration);
}
