use std::fs;

fn parse(contents: &str) -> Vec<Vec<char>> {
    contents
        .split('\n')
        .into_iter()
        .map(|str| str.chars().collect())
        .collect()
}

pub fn getRec(
    init: &Vec<Vec<char>>,
    (i, j): (i32, i32),
    next: fn(i32, i32) -> (i32, i32),
    size: usize,
    mut acc: Vec<char>,
) -> Option<String> {
    if size == 0 {
        Some(acc.iter().collect())
    } else if i >= 0 && j >= 0 && i < init.len() as i32 && j < init[i as usize].len() as i32 {
        acc.push(init[i as usize][j as usize]);
        getRec(init, next(i, j), next, size - 1, acc)
    } else {
        None
    }
}

pub fn day4_1() {
    let file_path = "4_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parsed: Vec<Vec<char>> = parse(&contents);
    let mut total = 0;
    for (i, row) in parsed.iter().enumerate().map(|(x, v)| (x as i32, v)) {
        for (j, x) in row.iter().enumerate().map(|(x, v)| (x as i32, v)) {
            if *x == 'X' {
                let count: i32 = [
                    getRec(&parsed, (i - 1, j), |n, m| (n - 1, m), 3, vec![]),
                    getRec(&parsed, (i, j - 1), |n, m| (n, m - 1), 3, vec![]),
                    getRec(&parsed, (i - 1, j - 1), |n, m| (n - 1, m - 1), 3, vec![]),
                    getRec(&parsed, (i + 1, j + 1), |n, m| (n + 1, m + 1), 3, vec![]),
                    getRec(&parsed, (i + 1, j), |n, m| (n + 1, m), 3, vec![]),
                    getRec(&parsed, (i, j + 1), |n, m| (n, m + 1), 3, vec![]),
                    getRec(&parsed, (i + 1, j - 1), |n, m| (n + 1, m - 1), 3, vec![]),
                    getRec(&parsed, (i - 1, j + 1), |n, m| (n - 1, m + 1), 3, vec![]),
                ]
                .iter()
                .fold(0, |acc, el| match el {
                    Some(ref s) if s == "MAS" => acc + 1,
                    _ => acc,
                });

                total = total + count;
            }
        }
    }

    println!("${:?}", total);
}

pub fn isX(init: &Vec<Vec<char>>, (i, j): (usize, usize)) -> bool {
    if i == 0 || j == 0 || i >= init.len() - 1 || j >= init[i].len() - 1 {
        false
    } else {
        let fst: String = [init[i - 1][j - 1], init[i][j], init[i + 1][j + 1]]
            .iter()
            .collect();
        let snd: String = [init[i - 1][j + 1], init[i][j], init[i + 1][j - 1]]
            .iter()
            .collect();

        (fst == "MAS" || fst == "SAM") && (snd == "MAS" || snd == "SAM")
    }
}

pub fn day4_2() {
    let file_path = "4_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parsed: Vec<Vec<char>> = parse(&contents);
    let mut total = 0;
    for (i, row) in parsed.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            if *x == 'A' && isX(&parsed, (i, j)) {
                total = total + 1;
            }
        }
    }

    println!("${:?}", total);
}
