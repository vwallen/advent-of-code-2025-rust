use colored::Colorize;
use adventofcode_2025::*;

#[allow(dead_code)]
static PENDING:&str    =  "🎁";
#[allow(dead_code)]
static COMPLETE:&str   =  "🌟";
#[allow(dead_code)]
static INCOMPLETE:&str =  "❄️";


fn main() {
    println!("{}", "\n\n🎄🎄🎄🎄 Advent of Code 2025 🎄🎄🎄🎄".bright_red());

    println!("{} {} {}", "----------".red(), "Day  1".bright_green(), "----------".red());
    println!("\tSecret Entrance");
    if let Ok(input) = day01::prepare("day01.txt") {
        let part1 = day01::part_1(&input);
        let part2 = day01::part_2(&input);
        println!("{}  {}", COMPLETE, part1.unwrap_or_default());
        println!("{}  {}", COMPLETE, part2.unwrap_or_default());
    }
/*
    println!("{} {} {}", "----------".red(), "Day  2".bright_green(), "----------".red());
    println!("\tTBD");
    if let Ok(input) = day02::prepare("day01.txt") {
        let part1 = day02::part_1(&input);
        let part2 = day02::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day  3".bright_green(), "----------".red());
    println!("\tTBD");
    if let Ok(input) = day03::prepare("day01.txt") {
        let part1 = day03::part_1(&input);
        let part2 = day03::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day  4".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day04::prepare("day01.txt") {
        let part1 = day04::part_1(&input);
        let part2 = day04::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day  5".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day05::prepare("day01.txt") {
        let part1 = day05::part_1(&input);
        let part2 = day05::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day  6".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day06::prepare("day01.txt") {
        let part1 = day06::part_1(&input);
        let part2 = day06::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day  7".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day07::prepare("day01.txt") {
        let part1 = day07::part_1(&input);
        let part2 = day07::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day  8".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day08::prepare("day01.txt") {
        let part1 = day08::part_1(&input);
        let part2 = day08::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day  9".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day09::prepare("day01.txt") {
        let part1 = day09::part_1(&input);
        let part2 = day09::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day 10".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day10::prepare("day01.txt") {
        let part1 = day10::part_1(&input);
        let part2 = day10::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day 11".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day11::prepare("day01.txt") {
        let part1 = day11::part_1(&input);
        let part2 = day11::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }

    println!("{} {} {}", "----------".red(), "Day 12".bright_green(), "----------".red());
    println!("\t🎁  TBD");
    if let Ok(input) = day12::prepare("day01.txt") {
        let part1 = day12::part_1(&input);
        let part2 = day12::part_2(&input);
        println!("{}  {}", PENDING, part1.unwrap_or_default());
        println!("{}  {}", PENDING, part2.unwrap_or_default());
    }
*/
    println!("{}", "============================".bright_red());
}
