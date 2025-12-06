use colored::Colorize;
use clap::Parser;
use adventofcode_2025::*;

#[allow(dead_code)]
static PENDING:&str    =  "🎁";
#[allow(dead_code)]
static COMPLETE:&str   =  "🌟";
#[allow(dead_code)]
static INCOMPLETE:&str =  "❄️";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// which day to run
    #[arg(short, long, default_value_t = 0)]
    day: u8,
}

fn main() {

    let args = Args::parse();

    println!("{}", "\n\n🎄🎄🎄🎄 Advent of Code 2025 🎄🎄🎄🎄".bright_red());

    if args.day == 1 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  1".bright_green(), "----------".red());
        println!("{: ^28}","Secret Entrance");
        if let Ok(input) = day01::prepare("day01.txt") {
            let part1 = day01::part_1(&input);
            let part2 = day01::part_2(&input);
            println!("{}  {}", COMPLETE, part1.unwrap_or_default());
            println!("{}  {}", COMPLETE, part2.unwrap_or_default());
        }
    }

    if args.day == 2 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  2".bright_green(), "----------".red());
        println!("{: ^28}","Gift Shop");
        if let Ok(input) = day02::prepare("day02.txt") {
            let part1 = day02::part_1(&input);
            let part2 = day02::part_2(&input);
            println!("{}  {}", COMPLETE, part1.unwrap_or_default());
            println!("{}  {}", COMPLETE, part2.unwrap_or_default());
        }
    }

    if args.day == 3 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  3".bright_green(), "----------".red());
        println!("{: ^28}","Lobby");
        if let Ok(input) = day03::prepare("day03.txt") {
            let part1 = day03::part_1(&input);
            let part2 = day03::part_2(&input);
            println!("{}  {}", COMPLETE, part1.unwrap_or_default());
            println!("{}  {}", COMPLETE, part2.unwrap_or_default());
        }
    }

    if args.day == 4 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  4".bright_green(), "----------".red());
        println!("{: ^28}", "Printing Department");
        if let Ok(input) = day04::prepare("day04.txt") {
            let part1 = day04::part_1(&input);
            let part2 = day04::part_2(&input);
            println!("{}  {}", COMPLETE, part1.unwrap_or_default());
            println!("{}  {}", COMPLETE, part2.unwrap_or_default());
        }
    }

    if args.day == 5 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  5".bright_green(), "----------".red());
        println!("{: ^28}", "Cafeteria");
        if let Ok(input) = day05::prepare("day05.txt") {
            let part1 = day05::part_1(&input);
            let part2 = day05::part_2(&input);
            println!("{}  {}", COMPLETE, part1.unwrap_or_default());
            println!("{}  {}", COMPLETE, part2.unwrap_or_default());
        }
    }

    if args.day == 6 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  6".bright_green(), "----------".red());
        println!("{: ^28}", "Trash Compactor");
        if let Ok(input) = day06::prepare("day06.txt") {
            let part1 = day06::part_1(&input);
            let part2 = day06::part_2(&input);
            println!("{}  {}", INCOMPLETE, part1.unwrap_or_default());
            println!("{}  {}", INCOMPLETE, part2.unwrap_or_default());
        }
    }

    if args.day == 7 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  7".bright_green(), "----------".red());
        println!("{: ^28}", PENDING);
        if let Ok(input) = day07::prepare("day07.txt") {
            let part1 = day07::part_1(&input);
            let part2 = day07::part_2(&input);
            println!("{}  {}", PENDING, part1.unwrap_or_default());
            println!("{}  {}", PENDING, part2.unwrap_or_default());
        }
    }

    if args.day == 8 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  8".bright_green(), "----------".red());
        println!("{: ^28}", PENDING);
        if let Ok(input) = day08::prepare("day08.txt") {
            let part1 = day08::part_1(&input);
            let part2 = day08::part_2(&input);
            println!("{}  {}", PENDING, part1.unwrap_or_default());
            println!("{}  {}", PENDING, part2.unwrap_or_default());
        }
    }

    if args.day == 9 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day  9".bright_green(), "----------".red());
        println!("{: ^28}", PENDING);
        if let Ok(input) = day09::prepare("day09.txt") {
            let part1 = day09::part_1(&input);
            let part2 = day09::part_2(&input);
            println!("{}  {}", PENDING, part1.unwrap_or_default());
            println!("{}  {}", PENDING, part2.unwrap_or_default());
        }
    }

    if args.day == 10 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day 10".bright_green(), "----------".red());
        println!("{: ^28}", PENDING);
        if let Ok(input) = day10::prepare("day10.txt") {
            let part1 = day10::part_1(&input);
            let part2 = day10::part_2(&input);
            println!("{}  {}", PENDING, part1.unwrap_or_default());
            println!("{}  {}", PENDING, part2.unwrap_or_default());
        }
    }

    if args.day == 11 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day 11".bright_green(), "----------".red());
        println!("{: ^28}", PENDING);
        if let Ok(input) = day11::prepare("day11.txt") {
            let part1 = day11::part_1(&input);
            let part2 = day11::part_2(&input);
            println!("{}  {}", PENDING, part1.unwrap_or_default());
            println!("{}  {}", PENDING, part2.unwrap_or_default());
        }
    }

    if args.day == 12 || args.day == 0 {
        println!("{} {} {}", "\n----------".red(), "Day 12".bright_green(), "----------".red());
        println!("{: ^28}", PENDING);
        if let Ok(input) = day12::prepare("day12.txt") {
            let part1 = day12::part_1(&input);
            let part2 = day12::part_2(&input);
            println!("{}  {}", PENDING, part1.unwrap_or_default());
            println!("{}  {}", PENDING, part2.unwrap_or_default());
        }
    }
    println!("{}", "\n============================".bright_red());
}
