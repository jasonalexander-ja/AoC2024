use lazy_static::lazy_static;
use std::fs;
use std::str::FromStr;
use regex::Regex;




fn main() {

    let s = fs::read_to_string("input.txt")
        .expect("Failed to read input. ")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let sum = process(&s);

    println!("Unfiltered sum {:?}", sum);
    
    
    let filtered = s.split("don't()").collect::<Vec<&str>>().iter()
        .map(|v| v.split("do()").collect::<Vec<&str>>())
        .enumerate()
        .map(|(i, v)| if i == 0 { v } else { v.into_iter().skip(1).collect::<Vec<&str>>() })
        .map(|v| v.into_iter().collect::<String>())
        .collect::<String>();

    let filtered_sum = process(&filtered);

    println!("Filtered sum {}", filtered_sum);
}


fn process(s: &String) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    }
    let matches = RE.find_iter(s)
        .map(|v| v.as_str().to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|v| v.replace("mul", "")
            .replace("(", "")
            .replace(")", ""))
        .collect::<Vec<String>>();
    matches.iter()
        .map(|v| v.split_once(","))
        .collect::<Option<Vec<(&str, &str)>>>()
        .expect("Failed to split parenthesized numbers. ")
        .into_iter()
        .map(|(left, right)| Ok::<(u32, u32), <u32 as FromStr>::Err>((left.parse::<u32>()?, right.parse::<u32>()?)))
        .collect::<Result<Vec<(u32, u32)>, <u32 as FromStr>::Err>>()
        .expect("Failed to parse parenthesized numbers. ")
        .into_iter()
        .map(|(l, r)| l * r)
        .sum()
}
