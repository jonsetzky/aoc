use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("src/bin/02/input.txt").expect("Failed to read file");

    for value in input_string.split(',').map(|s| {
        s.split("-")
            .map(|x| {
                x.trim()
                    .parse::<i64>()
                    .expect(format!("Failed to parse number: {}", x).as_str())
            })
            .collect::<Vec<_>>()
    }) {
        let [first, second]: [i64; 2] = value.try_into().expect("Expected exactly 2 entries");

        println!("{first} to {second}");
    }
}
