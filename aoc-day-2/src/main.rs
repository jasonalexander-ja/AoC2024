use std::fs;
use std::str::FromStr;


fn main() {
    let s = fs::read_to_string("input.txt").expect("Failed to read input. ");
    let strings = s.lines()
        .into_iter()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect::<Vec<Vec<&str>>>();
    let reports = strings.into_iter()
        .map(|s| s.into_iter()
            .map(|v| Ok::<i32, <i32 as FromStr>::Err>(v.parse::<i32>()?))
            .collect::<Result<Vec<i32>, <i32 as FromStr>::Err>>())
        .collect::<Result<Vec<Vec<i32>>, <i32 as FromStr>::Err>>()
        .expect("Failed to parse numbers. ");

    let safe = reports.iter()
        .map(|s| s.iter().take(s.len() - 1)
            .enumerate()
            .map(|(i, v)| s[i + 1] - *v)
            .collect::<Vec<i32>>())
        .filter(|s| s.iter()
            .all(|v| (v.abs() > 0 && v.abs() < 4) && s.iter().all(|c| c.is_negative() == v.is_negative())))
        .count();

    println!("Safe {}", safe);


    let dampened = reports.iter()
        .map(|s| (0..s.len()).map(|i| s.iter()
            .enumerate()
            .filter(|(vi, _)| *vi == i)
            .map(|(_, v)| *v)
            .collect::<Vec<i32>>()))
        .map(|u| u.map(|s| s.iter().take(s.len() - 1)
            .enumerate()
            .map(|(i, v)| s[i + 1] - *v)
            .collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>())
        .filter(|u| u.iter().any(|s| s.iter()
            .all(|v| (v.abs() > 0 && v.abs() < 4) && 
                s.iter().all(|c| c.is_negative() == v.is_negative()))))
        .count();

    println!("Dapened {}", dampened);

}


