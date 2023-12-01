use std::collections::HashMap;

fn process_line(s: &str) -> u32 {
    let values = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut leftmost_idx = s.len();
    let mut rightmost_idx: usize = 0;
    let mut leftmost_digit: u32 = 0;
    let mut rightmost_digit: u32 = 0;
    for (repr, val) in values {
        if let Some(cur_l_idx) = s.find(repr) {
            if cur_l_idx < leftmost_idx {
                leftmost_idx = cur_l_idx;
                leftmost_digit = val;
            }
        }
        if let Some(cur_r_idx) = s.rfind(repr) {
            if cur_r_idx >= rightmost_idx {
                rightmost_idx = cur_r_idx;
                rightmost_digit = val;
            }
        }
    }
    return leftmost_digit * 10 + rightmost_digit;
}
fn extract_first_digit(s: &str) -> u32 {
    let index = s.find(|c: char| c.is_digit(10)).unwrap();
    let digit_char = char::from_u32(u32::from(s.as_bytes()[index])).unwrap();
    return digit_char.to_digit(10).unwrap();
}
fn extract_last_digit(s: &str) -> u32 {
    let index = s.rfind(|c: char| c.is_digit(10)).unwrap();
    let digit_char = char::from_u32(u32::from(s.as_bytes()[index])).unwrap();
    return digit_char.to_digit(10).unwrap();
}

fn main() {
    let contents = std::fs::read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();
    let sum: u32 = lines.iter().map(|line| process_line(line)).sum();

    println!("the_sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_first_digit_only_digit() {
        assert_eq!(extract_first_digit("1"), 1);
    }
    #[test]
    fn extract_first_digit_mixed1() {
        assert_eq!(extract_first_digit("a1"), 1);
    }
    #[test]
    fn extract_first_digit_mixed2() {
        assert_eq!(extract_first_digit("1b"), 1);
    }
    #[test]
    fn extract_first_digit_two_digits() {
        assert_eq!(extract_first_digit("10"), 1);
    }
    #[test]
    fn extract_first_digit_two_digits2() {
        assert_eq!(extract_first_digit("01"), 0);
    }
    #[test]
    fn extract_last_digit_two_digits() {
        assert_eq!(extract_last_digit("21"), 1);
    }
    #[test]
    fn extract_last_digit_random_stuff() {
        assert_eq!(extract_last_digit("asa2sd4ak6lhj"), 6);
    }
    #[test]
    fn process_line_digits_only() {
        assert_eq!(process_line("12"), 12);
    }
    #[test]
    fn process_line_many_digits() {
        assert_eq!(process_line("1234567"), 17);
    }
    #[test]
    fn process_line_digit_and_spelled() {
        assert_eq!(process_line("1two"), 12);
    }
    #[test]
    fn process_line_spelled_and_digit() {
        assert_eq!(process_line("two1"), 21);
    }
    #[test]
    fn process_line_many_spelled() {
        assert_eq!(process_line("1two"), 12);
    }
    #[test]
    fn process_line_total_mix() {
        assert_eq!(
            process_line("asdadxone2three15adf3r1 3r13 asdacxxsa123"),
            13
        );
    }
    #[test]
    fn process_line_one_digit() {
        assert_eq!(process_line("1"), 11);
    }
}
