use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn parse(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let separator = Regex::new(r"\n|(\s+)").expect("invalid regex");

    let (left, right): (Vec<(usize, i32)>, Vec<(usize, i32)>) = separator
        .split(&contents)
        .into_iter()
        .map(|x| x.parse::<i32>().expect("invalid number"))
        .enumerate()
        .partition(|i| i.0 % 2 == 0); // partition_map...

    (
        left.into_iter().map(|x| x.1).collect(),
        right.into_iter().map(|x| x.1).collect(),
    )
}

pub fn day1_1() {
    let file_path = "1_1.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (mut left_els, mut right_els) = parse(&contents);

    left_els.sort();
    right_els.sort();

    let res = left_els
        .into_iter()
        .zip(right_els)
        .fold(0, |acc, (l, r)| acc + i32::abs(l - r));

    println!("{}", res);
}

pub fn day1_2() {
    let file_path = "1_1.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (left_els, right_els) = parse(&contents);

    let map: HashMap<i32, usize> = right_els.into_iter().counts_by(|x| x);
    let res = left_els
        .into_iter()
        .fold(0, |acc, x| acc + (x as usize) * map.get(&x).unwrap_or(&0));

    println!("{:?}", res);
}
