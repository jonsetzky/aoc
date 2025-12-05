use std::fs::read_to_string;

fn is_invalid_id(id: i64) -> bool {
    let dcount = (id as f64).log10().ceil() as u32;
    let mut exp = 1;

    while exp <= (dcount / 2) {
        // ! only sequences that are repeated twice count
        if dcount / exp > 2 {
            exp += 1;
            continue;
        }
        // skip digit sequences that cant possibly add up to dcount
        if dcount % exp != 0 {
            exp += 1;
            continue;
        }

        let seq = id % 10i64.pow(exp);
        if seq == 0 {
            exp += 1;
            continue;
        }

        let mut found = true;
        for i in 0..(dcount / exp) {
            let next_seq = id / 10i64.pow(exp * i) % 10i64.pow(exp);
            if cfg!(test) {
                println!("Checking seq {} at exp {}, next {}", seq, exp * i, next_seq);
            }
            if next_seq != seq {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }

        if id % seq != 0 {}

        exp += 1;
    }
    false
}

fn find_invalid_ids(start: i64, end: i64) -> impl Iterator<Item = i64> {
    (start..=end).filter(|&id| is_invalid_id(id))
}

fn main() {
    let input_string = read_to_string("src/bin/02/input.txt").expect("Failed to read file");
    let mut sum = 0;

    for value in input_string.split(',').map(|s| {
        s.split("-")
            .map(|x| {
                x.trim()
                    .parse::<i64>()
                    .expect(format!("Failed to parse i64ber: {}", x).as_str())
            })
            .collect::<Vec<_>>()
    }) {
        let [first, second]: [i64; 2] = value.try_into().expect("Expected exactly 2 entries");

        find_invalid_ids(first, second).for_each(|id| {
            sum += id;
        });
    }

    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use test_case::test_case;

    #[test_case(111111)]
    #[test_case(99)]
    #[test_case(1010)]
    fn is_valid_id_valid_cases(id: i64) {
        assert!(is_invalid_id(id));
    }

    #[test_case(111112)]
    #[test_case(1188511880)]
    fn is_valid_id_invalid_cases(id: i64) {
        assert!(!is_invalid_id(id));
    }

    #[test_case(11, 22, vec!(11, 22) ; "range with invalid IDs 1")]
    #[test_case(95, 115, vec!(99) ; "range with invalid IDs 2")]
    #[test_case(998, 1012, vec!(1010) ; "range with invalid IDs 3")]
    #[test_case(1188511880, 1188511890, vec!(1188511885) ; "range with invalid IDs 4")]
    #[test_case(222220, 222224, vec!(222222) ; "range with invalid IDs 5")]
    #[test_case(1698522, 1698528, vec!() ; "range with invalid IDs 6")]
    #[test_case(446443, 446449, vec!(446446) ; "range with invalid IDs 7")]
    #[test_case(38593856, 38593862, vec!(38593859) ; "range with invalid IDs 8")]
    fn it_works(start: i64, end: i64, expected: Vec<i64>) {
        let actual: Vec<i64> = find_invalid_ids(start, end).collect();
        println!(
            "Actual: {:?}",
            actual.iter().map(|f| f.to_string()).collect::<Vec<_>>()
        );
        assert_eq!(true, expected.iter().cloned().eq(actual.iter().cloned()));
    }
}
