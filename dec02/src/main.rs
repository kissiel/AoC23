type RGB = (u32, u32, u32);

fn merge_rgbs(rgb1: RGB, rgb2: RGB) -> RGB {
    (rgb1.0 + rgb2.0, rgb1.1 + rgb2.1, rgb1.2 + rgb2.2)
}

fn parse_set_into_rgb(set_description: &str) -> RGB {
    let mut whole_set: RGB = (0, 0, 0);

    for single_description in set_description.split(", ") {
        whole_set = merge_rgbs(whole_set, parse_description(single_description));
    }
    whole_set
}

fn parse_description(description: &str) -> RGB {
    // this is for one ball only

    let (count, color) = description.split_once(' ').unwrap_or(("", ""));
    let proper_count = u32::from_str_radix(count, 10).unwrap_or(0);
    match color {
        "red" => (proper_count, 0, 0),
        "green" => (0, proper_count, 0),
        "blue" => (0, 0, proper_count),
        _ => (0, 0, 0),
    }
}
fn split_game_into_sets(game: &str) -> impl Iterator<Item = RGB> + '_ {
    let (game_id, all_sets) = game.split_once(": ").unwrap();

    let mapping = all_sets.split("; ").into_iter().map(parse_set_into_rgb);
    mapping.into_iter()

    //.for_each(parse_set_into_rgb)

    //    .map(parse_set_into_rgb(set_description)).
}
fn is_within_limits(limit: RGB, set: RGB) -> bool {
    set.0 <= limit.0 && set.1 <= limit.1 && set.2 <= limit.2
}

fn main() {
    let contents = std::fs::read_to_string("input1.txt").unwrap();
    let mut sum: usize = 0;
    for (index, line) in contents.split('\n').enumerate() {
        let limits: RGB = (12, 13, 14);

        if split_game_into_sets(line).all(|set| is_within_limits(limits, set)) {
            sum += index + 1;
        }
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set_into_rgb_none() {
        let input = "";
        let expected = (0, 0, 0);
        assert_eq!(parse_set_into_rgb(input), expected);
    }
    #[test]
    fn test_parse_set_into_only_red() {
        let input = "1 red";
        let expected = (1, 0, 0);
        assert_eq!(parse_set_into_rgb(input), expected);
    }
    #[test]
    fn test_parse_set_into_only_blue() {
        let input = "1 blue";
        let expected = (0, 0, 1);
        assert_eq!(parse_set_into_rgb(input), expected);
    }
    #[test]
    fn test_parse_set_into_red_and_blue() {
        let input = "1 red, 1 blue";
        let expected = (1, 0, 1);
        assert_eq!(parse_set_into_rgb(input), expected);
    }
    #[test]
    fn test_parse_set_into_red_and_blue_inverse() {
        let input = "1 blue, 1 red";
        let expected = (1, 0, 1);
        assert_eq!(parse_set_into_rgb(input), expected);
    }
    #[test]
    fn test_parse_set_into_one_each() {
        let input = "1 blue, 1 red, 1 green";
        let expected = (1, 1, 1);
        assert_eq!(parse_set_into_rgb(input), expected);
    }
    #[test]
    fn test_parse_description_none() {
        let input = "";
        let expected = (0, 0, 0);
        assert_eq!(parse_description(input), expected);
    }
    #[test]
    fn test_parse_description_1_red() {
        let input = "1 red";
        let expected = (1, 0, 0);
        assert_eq!(parse_description(input), expected);
    }
    #[test]
    fn test_parse_description_2_blue() {
        let input = "2 blue";
        let expected = (0, 0, 2);
        assert_eq!(parse_description(input), expected);
    }
    #[test]
    fn test_parse_description_3_blue() {
        let input = "3 green";
        let expected = (0, 3, 0);
        assert_eq!(parse_description(input), expected);
    }
    #[test]
    fn test_parse_description_explicit_0() {
        let input = "0 green";
        let expected = (0, 0, 0);
        assert_eq!(parse_description(input), expected);
    }
    #[test]
    fn test_merge_rgbs_smoke() {
        let one: RGB = (1, 2, 3);
        let the_other: RGB = (4, 5, 6);
        assert_eq!(merge_rgbs(one, the_other), (5, 7, 9));
    }
}
