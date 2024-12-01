use std::fs;
use std::str::FromStr;


fn main() {
    let s = fs::read_to_string("input.txt").expect("Failed to read input. ");
    let (mut left, mut right) = s.lines()
        .into_iter()
        .map(|line| line.split_once("   "))
        .collect::<Option<Vec<(&str, &str)>>>()
        .expect("Each item should contain 2 numbers separated by 3 spaces. ")
        .into_iter()
        .map(|(left, right)| Ok::<(u64, u64), <u64 as FromStr>::Err>((left.parse::<u64>()?, right.parse::<u64>()?)))
        .collect::<Result<Vec<(u64, u64)>, <u64 as FromStr>::Err>>()
        .expect("Failed to parse numbers. ")
        .into_iter()
        .collect::<(Vec<u64>, Vec<u64>)>();

    left.sort();
    right.sort();
    let distance: u64 = left.iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum();

    println!("Distance {}", distance);

    let similarity: usize = left.iter()
        .map(|l| (*l as usize) * right.iter().filter(|r| *r == l).count())
        .sum();

    println!("Similarity {}", similarity);

}
