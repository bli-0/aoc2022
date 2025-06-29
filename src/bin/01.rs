use aoc2022::time_run;

const INPUT: &str = include_str!("../inputs/01");

#[time_run("01")]
fn main() {
    elves(INPUT)
}

fn elves(i: &str) -> String {
    let mut totals: Vec<u64> = i
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calorie| calorie.parse::<u64>().unwrap())
                .sum()
        })
        .collect();
    totals.sort_by(|a, b| b.cmp(a));

    let top_3: Vec<u64> = totals.into_iter().take(3).collect();
    let top_3_total: u64 = top_3.iter().sum();

    top_3_total.to_string()
}
