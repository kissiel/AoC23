use std::collections::HashMap;

type Game = (u32, [u32; 10], [u32; 25]);

fn parse_numbers(text: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for s in text.split(' ') {
        if s.is_empty() {
            continue;
        }
        numbers.push(s.trim().parse().unwrap());
    }
    numbers
}

fn parse_line(line: &str) -> Game {
    let (game_id, numbers) = line.split_once(':').unwrap();
    let (winning_part, elf_numbers) = numbers.split_once('|').unwrap();

    let winning_nums = parse_numbers(winning_part);

    let elf_nums = parse_numbers(elf_numbers);
    let (_, game_num_s) = game_id.split_once(' ').unwrap();
    let game_num: u32 = game_num_s.trim().parse().unwrap();
    (
        game_num,
        winning_nums.try_into().unwrap(),
        elf_nums.try_into().unwrap(),
    )
}

fn how_many_common(v1: &[u32; 10], v2: &[u32; 25]) -> u32 {
    let mut count: u32 = 0;

    for number in v1 {
        if v2.contains(&number) {
            count += 1;
        }
    }
    count
}
fn main() {
    let contents = std::fs::read_to_string("input1.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut scratchcard_counter: HashMap<u32, u32> = HashMap::new();

    let mut grand_total: u32 = 0;
    for line in lines {
        let (game_id, lucky, elf) = parse_line(line);
        if !scratchcard_counter.contains_key(&game_id) {
            scratchcard_counter.insert(game_id, 1);
        } else {
            scratchcard_counter.entry(game_id).and_modify(|e| *e += 1);
        }

        let good_ones = how_many_common(&lucky, &elf);
        println!("{good_ones}");
        if good_ones > 0 {
            let start = game_id + 1;
            let stop = game_id + good_ones + 1;
            for i in start..stop {
                if !scratchcard_counter.contains_key(&i) {
                    scratchcard_counter.insert(i, 0);
                }

                let how_many_times = scratchcard_counter.get(&game_id).unwrap().clone();

                scratchcard_counter
                    .entry(i)
                    .and_modify(|e| *e += how_many_times);
            }
        }
    }
    println!("{:?}", scratchcard_counter);
    for (k, v) in scratchcard_counter {
        grand_total += v;
    }
    println!("{grand_total}");
}
