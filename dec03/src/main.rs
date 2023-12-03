fn main() {
    phase2_main();
    return;
    let contents = std::fs::read_to_string("input1.txt").unwrap();
    let all_lines: Vec<(usize, &str)> = contents.split('\n').enumerate().collect();

    let mut sum: i32 = 0;

    for (line_no, line) in all_lines.clone() {
        for descriptor in find_numbers(line) {
            let start_idx: usize = descriptor.1.checked_sub(1).unwrap_or(0);
            let end_idx: usize = std::cmp::min(descriptor.2 + 1, line.len());
            if line_no != 0 {
                if has_symbol_in_range(all_lines[line_no - 1].1, start_idx, end_idx) {
                    sum += descriptor.0;
                    continue;
                }
            }
            if has_symbol_in_range(all_lines[line_no].1, start_idx, end_idx) {
                sum += descriptor.0;
                continue;
            }
            if line_no < all_lines.len() - 1 {
                if has_symbol_in_range(all_lines[line_no + 1].1, start_idx, end_idx) {
                    sum += descriptor.0;
                    continue;
                }
            }
        }
    }
    println!("{}", sum);
}

fn check_overlap(a: usize, b: usize, c: usize, d: usize) -> bool {
    std::cmp::max(a, c) <= std::cmp::min(b, d)

    //(b >= c && d >= b) || (a >= c && d >= a)
}

fn get_sum_of_overlaping_numbers(text: &str, range: (usize, usize)) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for descriptor in find_numbers(text) {
        let a = range.0;
        let b = range.1;
        let c = descriptor.1;
        let d = descriptor.2 - 1;

        if check_overlap(a, b, c, d) {
            numbers.push(descriptor.0);
        }
    }
    numbers
}

fn phase2_main() {
    let contents = std::fs::read_to_string("input1.txt").unwrap();
    let all_lines: Vec<(usize, &str)> = contents.split('\n').enumerate().collect();

    let mut sum: i32 = 0;

    for (line_no, line) in all_lines.clone() {
        for (idx, c) in line.char_indices() {
            if c != '*' {
                continue;
            }
            let mut all_gears: Vec<i32> = Vec::new();
            let range: (usize, usize) = (
                idx.checked_sub(1).unwrap_or(0),
                std::cmp::min(idx + 1, line.len()),
            );
            if line_no != 0 {
                // check for numbers above
                all_gears.append(
                    get_sum_of_overlaping_numbers(all_lines[line_no - 1].1, range).as_mut(),
                );
            }
            all_gears.append(get_sum_of_overlaping_numbers(all_lines[line_no].1, range).as_mut());
            if line_no < all_lines.len() - 1 {
                all_gears.append(
                    get_sum_of_overlaping_numbers(all_lines[line_no + 1].1, range).as_mut(),
                );
            }
            if all_gears.len() != 2 {
                continue;
            }
            sum += all_gears[0] * all_gears[1];
        }
    }

    println!("{}", sum);
}

fn has_symbol_in_range(text: &str, start: usize, stop: usize) -> bool {
    let the_slice = &text[start..stop];
    the_slice.chars().any(|c| c != '.' && !c.is_digit(10))
}

type Descriptor = (i32, usize, usize);

fn find_numbers(text: &str) -> Vec<Descriptor> {
    let mut number_descriptions: Vec<Descriptor> = Vec::new();
    let mut current_number: i32 = 0;
    let mut begin_idx: usize = 0;

    for (idx, c) in text.chars().enumerate() {
        match c {
            '0'..='9' => {
                let the_digit = c.to_digit(10).unwrap() as i32;
                if current_number > 0 {
                    current_number = current_number * 10 + the_digit;
                } else {
                    begin_idx = idx;
                    current_number = the_digit;
                }
            }
            '.' | _ => {
                if current_number > 0 {
                    number_descriptions.push((current_number, begin_idx, idx));

                    current_number = 0;
                }
            }
        }
    }
    if current_number > 0 {
        number_descriptions.push((current_number, begin_idx, text.len()));
    }

    number_descriptions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_numbers_one_digit() {
        let expected = vec![(6, 0, 1)];
        assert_eq!(find_numbers("6"), expected);
    }

    #[test]
    fn test_find_numbers_many_numbers() {
        let expected = vec![(42, 2, 4), (1, 6, 7), (42, 10, 12)];
        assert_eq!(find_numbers("..42*.1..&42"), expected);
    }

    #[test]
    fn has_symbol_in_range_indeed() {
        assert!(has_symbol_in_range("..#..", 1, 4));
    }
    #[test]
    fn has_symbol_in_range_nope() {
        assert!(!has_symbol_in_range("..#..", 0, 2));
    }

    #[test]
    fn check_overlap_1() {
        assert!(check_overlap(2, 5, 5, 6));
    }

    #[test]
    fn check_overlap_2() {
        assert!(check_overlap(1, 1, 1, 1));
    }
    #[test]
    fn check_overlap_3() {
        assert!(check_overlap(1, 2, 0, 1));
    }
    #[test]
    fn check_overlap_4() {
        assert!(check_overlap(1, 5, 2, 3));
    }
    #[test]
    fn check_overlap_5() {
        assert!(check_overlap(5, 8, 2, 6));
    }
}
