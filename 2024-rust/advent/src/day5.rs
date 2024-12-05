use std::{cmp::Ordering, collections::HashMap, fs};

use itertools::Itertools;

fn parse(contents: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let parts: Vec<&str> = contents.split("\n\n").into_iter().collect();

    let rules: Vec<(i32, i32)> = parts[0]
        .split("\n")
        .map(|r| {
            let n: Vec<&str> = r.split("|").collect();
            (n[0].parse::<i32>().unwrap(), n[1].parse::<i32>().unwrap())
        })
        .collect();

    let input: Vec<Vec<i32>> = parts[1]
        .split("\n")
        .map(|r| r.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    (rules, input)
}

pub fn day5_1() {
    let file_path = "5_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (rules, input) = parse(&contents);
    println!("{:?}", rules);
    println!("{:?}", input);

    let res = input.into_iter().fold(0, |acc, i| {
        let values: HashMap<&i32, usize> = i.iter().enumerate().map(|(x, y)| (y, x)).collect();
        println!("{:?}", values);
        let check = rules
            .iter()
            .all(|(fst, snd)| match (values.get(&fst), values.get(&snd)) {
                (Some(n), Some(m)) if n >= m => false,
                _ => true,
            });
        if check {
            acc + i[i.len() / 2]
        } else {
            acc
        }
    });

    println!("${:?}", res);
}

pub fn day5_2() {
    let file_path = "5_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (rules, input) = parse(&contents);
    println!("{:?}", rules);
    println!("{:?}", input);

    let res = input
        .into_iter()
        .filter(|i| {
            let values: HashMap<&i32, usize> = i.iter().enumerate().map(|(x, y)| (y, x)).collect();
            rules
                .iter()
                .any(|(fst, snd)| match (values.get(&fst), values.get(&snd)) {
                    (Some(n), Some(m)) if n >= m => true,
                    _ => false,
                })
        })
        .map(|i| {
            let res: Vec<i32> = i
                .iter()
                .sorted_by(|x, y| {
                    if rules.contains(&(**x, **y)) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                })
                .map(|x| *x)
                .collect();
            res
        })
        .fold(0, |acc, i| acc + i[i.len() / 2]);

    println!("${:?}", res);
}
