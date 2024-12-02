use itertools::Itertools;
use std::fs;

fn parse(contents: &str) -> Vec<Vec<i32>> {
    contents
        .split('\n')
        .into_iter()
        .map(|x| {
            x.split(' ')
                .into_iter()
                .map(|n| n.parse::<i32>().expect("invalid number"))
                .collect()
        })
        .collect()
}

pub fn check_diffs(diffs: &Vec<i32>) -> bool {
    println!("checking diffs {:?}", diffs);
    let res = if diffs.is_empty() {
        true
    } else {
        let head = diffs.get(0).unwrap();
        let tail: Vec<i32> = diffs[1..].to_vec();

        if head == &0 || head.abs() > 3 {
            false
        } else if head > &0 {
            tail.iter().all(|e| (e > &0) && e.abs() <= 3)
        } else {
            tail.iter().all(|e| (e < &0) && e.abs() <= 3)
        }
    };
    if res {
        println!("{:?}", res)
    };
    res
}

pub fn day2_1() {
    let file_path = "2_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let res: usize = parse(&contents)
        .iter()
        .map(|v| {
            println!("{:?}", v);
            let diffs: Vec<i32> = v
                .iter()
                .zip(v[1..].iter().copied())
                .map(|(l, r)| l - r)
                .collect();

            println!("{:?}", diffs);
            check_diffs(&diffs)
        })
        .filter(|x| x == &true)
        .count();

    println!("{:?}", res);
}

pub fn check_diffs_can_be_fixed(diffs: Vec<i32>) -> bool {
    check_diffs(&diffs[1..].to_vec())
        || check_diffs(&diffs[0..(diffs.len() - 1)].to_vec())
        || (1..(diffs.len() - 1))
            .map(|x| {
                let mut fixed: Vec<i32> = diffs.iter().copied().collect_vec();
                let rd: i32 = fixed.remove(x);
                fixed[x] = rd + fixed[x];
                fixed
            })
            .any(|g| check_diffs(&g))
        || (1..(diffs.len() - 1))
            .map(|x| {
                let mut fixed: Vec<i32> = diffs.iter().copied().collect_vec();
                let rd: i32 = fixed.remove(x);
                fixed[x - 1] = rd + fixed[x - 1];
                fixed
            })
            .any(|g| check_diffs(&g))
}

pub fn day2_2() {
    let file_path = "2_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let res: usize = parse(&contents)
        .iter()
        .map(|v| {
            println!("values {:?}", v);
            let diffs: Vec<i32> = v
                .iter()
                .zip(v[1..].iter().copied())
                .map(|(l, r)| l - r)
                .collect();

            println!("diffs {:?}", diffs);

            check_diffs_can_be_fixed(diffs)
        })
        .filter(|x| x == &true)
        .count();

    println!("{:?}", res);
}
