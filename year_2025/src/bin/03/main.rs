use std::{cmp::min, fs::read_to_string};

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

fn find_max_joltage_v2(bank: &str) -> String {
    let mut out = String::new();
    let max_len = min(12, bank.len());

    // bank.get(0..(bank.len() - max_len + 1))
    // gets all possible characters that the first character can be

    let mut l = 0;
    let mut r = bank.len();

    for _ in 0..12 {
        let possible = bank.get(l..(r - max_len + 1)).unwrap();

        let max = possible.chars().max().unwrap();
        let max_index = possible.chars().position(|c| c == max).unwrap() + l;

        // println!(
        //     "{}, max {} ({})",
        //     bank.get(l..(r - max_len + 1)).unwrap(),
        //     max,
        //     max_index
        // );

        l = max_index + 1;
        r += 1;
        out.push(max);
    }

    // println!("Final: {}", out);

    return out;
}

fn main() {
    let input_string = read_to_string("src/bin/03/input.txt").expect("Failed to read file");
    let mut sum = 0;
    let mut sum_v2 = 0;

    for line in input_string.lines() {
        sum += find_max_joltage(line);
        sum_v2 += find_max_joltage_v2(line).parse::<u64>().unwrap_or(0);
    }

    println!("Sum: {}", sum);
    println!("Sum v2: {}", sum_v2);
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

    #[test_case("987654321111111", "987654321111".to_string())]
    #[test_case("811111111111119", "811111111119".to_string())]
    #[test_case("234234234234278", "434234234278".to_string())]
    #[test_case("818181911112111", "888911112111".to_string())]
    fn it_works_v2(bank: &str, expected: String) {
        assert_eq!(
            true,
            expected.eq(find_max_joltage_v2(bank).to_string().as_str())
        );
    }
}
