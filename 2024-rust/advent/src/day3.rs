use regex::Regex;
use std::fs;

fn parse_mul(contents: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut results: Vec<(u64, u64)> = vec![];
    for (_, [left, right]) in re.captures_iter(contents).map(|c| c.extract()) {
        results.push((left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()));
    }

    results
}

pub fn day3_1() {
    let file_path = "3_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parsed: Vec<(u64, u64)> = parse_mul(&contents);
    let res = parsed.iter().fold(0, |acc, (l, r)| acc + l * r);
    println!("${:?}", parsed);
    println!("${:?}", res);
}

enum Inst {
    Mul { l: u64, r: u64 },
    Disable,
    Enable,
}

fn parse_mul_and_do(contents: &str) -> Vec<Inst> {
    let do_regex = Regex::new(r"don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();

    do_regex
        .find_iter(contents)
        .map(|m| m.as_str())
        .map(|s| match s {
            "do()" => Inst::Enable,
            "don't()" => Inst::Disable,
            _ => {
                let (l_str, r_str) = s[4..s.len() - 1].split_once(',').unwrap();
                let l = l_str.parse::<u64>().unwrap();
                let r = r_str.parse::<u64>().unwrap();
                Inst::Mul { l: l, r: r }
            }
        })
        .collect()
}

pub fn day3_2() {
    let file_path = "3_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parsed: Vec<Inst> = parse_mul_and_do(&contents);
    let res = parsed
        .iter()
        .fold((true, 0), |(state, acc), inst| match inst {
            Inst::Mul { l, r } => {
                if state {
                    (state, acc + l * r)
                } else {
                    (state, acc)
                }
            }
            Inst::Disable => (false, acc),
            Inst::Enable => (true, acc),
        });

    println!("${:?}", res);
}
