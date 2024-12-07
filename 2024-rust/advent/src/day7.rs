use core::num;
use std::{cmp::Ordering, collections::HashMap, fs, usize};

use itertools::Itertools;
use std::borrow::BorrowMut;

#[derive(Clone, Copy, Debug)]
enum Op {
    Mul,
    Sum,
}

fn parse(contents: &str) -> Vec<(i64, Vec<i64>)> {
    let parts: Vec<&str> = contents.split("\n").into_iter().collect();

    parts
        .into_iter()
        .map(|i| {
            let s: Vec<&str> = i.split(": ").into_iter().collect();
            let (res, numbers) = (s[0], s[1]);
            println!("{:?}, {:?}", res, numbers);
            let numbersParsed: Vec<i64> = numbers
                .split(" ")
                .into_iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            (res.parse::<i64>().unwrap(), numbersParsed)
        })
        .collect()
}

fn couldBeTrue(res: i64, mut values: Vec<i64>) -> bool {
    let last = values.pop();
    match last {
        Some(x) => {
            ((res % x == 0) && couldBeTrue(res / x, values.clone())) || couldBeTrue(res - x, values)
        }
        None => res == 0,
    }
}

fn splitBy(l: i64, end: i64) -> Option<i64> {
    let tail = end.to_string();
    println!("{:?}", end);
    if l.to_string().ends_with(&tail) {
        println!("{:?}", l - end);
        println!("{:?}", tail.len() as i64);

        Some((l - end) / (10 as i64).pow(tail.len() as u32))
    } else {
        None
    }
}

fn couldBeTrue2(res: i64, mut values: Vec<i64>) -> bool {
    let last = values.pop();
    println!("last: {:?}", last);
    match last {
        Some(x) => {
            ((res % x == 0) && couldBeTrue2(res / x, values.clone()))
                || couldBeTrue2(res - x, values.clone())
                || ((splitBy(res, x).map(|r| couldBeTrue2(r, values.clone()))).unwrap_or(false))
        }
        None => res == 0,
    }
}

pub fn day7_1() {
    let file_path = "7_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let eqs: Vec<(i64, Vec<i64>)> = parse(&contents);
    let res = eqs.iter().fold(0, |acc, eq| {
        if couldBeTrue2(eq.0, eq.1.clone()) {
            acc + eq.0
        } else {
            acc
        }
    });
    println!("{:?}", res);
}
