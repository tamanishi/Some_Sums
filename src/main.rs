use std::io;

fn main() {
    let mut line1 = String::new();
    let _ = io::stdin().read_line(&mut line1);
    let values = line1
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let n = values[0];
    let a = values[1];
    let b = values[2];

    let mut candidates: Vec<u32> = vec![];

    for i in 1..=n {
        let sum: u32 = i
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|e| e.to_digit(10).unwrap())
            .sum::<u32>();
        if sum >= a && sum <= b {
            candidates.push(i);
        }
    }

    println!("{}", candidates.iter().sum::<u32>());
}
