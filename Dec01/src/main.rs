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
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();
    let sum: u32 = lines
        .iter()
        .map(|line| extract_first_digit(line) * 10 + extract_last_digit(line))
        .sum();

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
}
