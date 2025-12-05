use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("src/bin/01/input.txt").expect("Failed to read file");

    let mut dial = 50;
    let mut zero_count = 0;

    for line in input_string.lines() {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let value: i32 = chars.collect::<String>().parse().unwrap();

        match dir {
            'L' => dial -= value,
            'R' => dial += value,
            _ => (),
        }

        if dial < 0 {
            dial += 100 * ((-(dial as f64) / 100.0).floor() as i32 + 1);
        }

        dial %= 100;

        if dial == 0 {
            zero_count += 1;
        }
    }
    println!("Final dial position: {}", dial);
    println!("Number of times dial was at 0: {}", zero_count);
}
