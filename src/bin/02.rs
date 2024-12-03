use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

/*
1 3 2 4 5
 */
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let safe_count = reader.lines()
            .map(Result::unwrap)
            .map(|l| {
                let mut pairs = l.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap())
                    .tuple_windows::<(_, _)>();
                let mut pairs_clone = pairs.clone();

                pairs.all(|(a, b)| b > a && b - a <= 3) ||
                    pairs_clone.all(|(a, b)| a > b && a - b <= 3)
            })
            .filter(|&safe| safe)
            .count();

        Ok(safe_count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    /*

     */
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let safe_count = reader.lines()
            .map(Result::unwrap)
            .map(|l| {
                let elements = l.split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                let safe = (0..elements.len()).map(|i| {
                    let mut it = elements.iter()
                        .enumerate()
                        .filter(|&(idx, _)| idx != i)
                        .map(|(_, &v)| v)
                        .tuple_windows::<(_, _)>();

                    it.clone().all(|(a, b)| b > a && b - a <= 3) ||
                        it.all(|(a, b)| a > b && a - b <= 3)
                }).any(|safe| safe);

                safe
            })
            .filter(|&safe| safe)
            .count();

        Ok(safe_count)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
