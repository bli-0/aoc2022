use aoc2022::time_run2;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/20");

#[time_run2("20")]
fn main() {
    grove_positioning_system(INPUT)
}

fn grove_positioning_system(i: &str) -> (String, String) {
    let items = i.lines().map(|l| l.parse::<i64>().unwrap()).collect_vec();
    let mut p1_answer = items.clone();

    for item in items.iter() {
        let (index, _) = p1_answer
            .iter()
            .enumerate()
            .find(|(_, i)| *i == item)
            .unwrap();

        if *item >= 0 {
            for i in 0..*item {
                let a = (index + i as usize).rem_euclid(items.len());
                let b = (index + i as usize + 1).rem_euclid(items.len());
                p1_answer.swap(a, b)
            }
        } else {
            for i in 0..item.abs() {
                let a = (index as i64 - i).rem_euclid(items.len() as i64);
                let b = (index as i64 - i - 1).rem_euclid(items.len() as i64);

                p1_answer.swap(a as usize, b as usize)
            }
        }
        //p1_answer = dbg!(p1_answer);
    }

    let (index_of_0, _) = p1_answer
        .iter()
        .enumerate()
        .find(|(_, item)| **item == 0)
        .unwrap();
    let mut part1 = 0;
    for i in [1000, 2000, 3000] {
        part1 += p1_answer[(index_of_0 + i as usize).rem_euclid(items.len())]
    }

    (part1.to_string(), "".to_string())
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let a = (100 -9001_i64).rem_euclid(1000);
        let b = (100 -9001_i64 - 1).rem_euclid(1000);

        let c= (500_i64 + 500).rem_euclid(1000);
        let d = (500_i64 + 500 + 1).rem_euclid(1000);

        dbg!(a);
        dbg!(b);
        dbg!(c);
        dbg!(d);

        assert_eq!(a - 1, b);
    }
}
