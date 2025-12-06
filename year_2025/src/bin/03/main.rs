use std::fs::read_to_string;

fn find_max_joltage(bank: &str) -> i64 {
    for n in (1..=9).rev() {
        let l = bank.find(&format!("{}", n));
        if l.is_none() {
            continue;
        }

        for m in (1..=9).rev() {
            let r = bank[(l.unwrap() + 1)..].find(&format!("{}", m));
            if r.is_none() {
                continue;
            }

            return format!("{}{}", n, m).parse::<i64>().unwrap_or(0i64);
        }
    }

    bank.chars()
        .take(2)
        .collect::<String>()
        .parse::<i64>()
        .unwrap_or(0)
}

fn main() {
    let input_string = read_to_string("src/bin/03/input.txt").expect("Failed to read file");
    let mut sum = 0;

    for line in input_string.lines() {
        sum += find_max_joltage(line);
    }

    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("987654321111111", "98".to_string())]
    #[test_case("811111111111119", "89".to_string())]
    #[test_case("234234234234278", "78".to_string())]
    #[test_case("818181911112111", "92".to_string())]
    fn it_works(bank: &str, expected: String) {
        assert_eq!(
            true,
            expected.eq(find_max_joltage(bank).to_string().as_str())
        );
    }
}
