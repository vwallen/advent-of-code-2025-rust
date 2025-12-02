use adventofcode_2025::day02::*;

const INPUT:&str = "day02.txt";

fn main() {
    divan::main();
}

#[divan::bench(name="part 1", sample_count=10, sample_size=10)]
fn bench_part_1() {
    if let Ok(input) = prepare(INPUT) {
        part_1(&input);
    }
}

#[divan::bench(name="part 2", sample_count=10, sample_size=10)]
fn bench_part_2() {
    if let Ok(input) = prepare(INPUT) {
        part_2(&input);
    }
}