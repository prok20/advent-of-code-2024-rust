use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3 4
4 3
2 5
1 3
3 9
3 3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let (first, second) = parse_two_lists(reader);

        let diff_sum = first
            .into_iter()
            .sorted_unstable()
            .zip(second.into_iter().sorted_unstable())
            .map(|(x, y)| {
                let diff = x.checked_sub(y).unwrap_or_else(|| y - x);
                diff as u64
            })
            .sum::<u64>();

        Ok(diff_sum)
    }

    fn parse_two_lists<R: BufRead>(reader: R) -> (Vec<u32>, Vec<u32>) {
        let (first, second): (Vec<_>, Vec<_>) = reader
            .lines()
            .map(|l| l.unwrap().trim_ascii().to_string())
            .filter(|l| !l.is_empty())
            .map(|line| {
                let pair = line
                    .split_ascii_whitespace()
                    .collect_tuple::<(_, _)>()
                    .unwrap();
                (
                    pair.0.parse::<u32>().unwrap(),
                    pair.1.parse::<u32>().unwrap(),
                )
            })
            .unzip();
        (first, second)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut counter = HashMap::<u32, u32>::new();
        let (first, second) = parse_two_lists(reader);

        second
            .iter()
            .for_each(|&x| *counter.entry(x).or_insert(0) += 1);

        let similarity_score: u64 = first.iter().map(|x| {
            let times_in_second = counter.get(x).unwrap_or(&0).clone();
            (x.clone() as u64) * (times_in_second as u64)
        }).sum();

        Ok(similarity_score)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
