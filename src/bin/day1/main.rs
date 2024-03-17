

fn main() {

    let contents = std::include_str!("input");

    let mut sum: u64 = 0;
    let mut sum2: u64 = 0;
    for line in contents.lines() {
        if line.len() == 0 { continue; }
        let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let second = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        sum += format!("{first}{second}").parse::<u64>().unwrap();
        let first_2 = first_number(line);
        let second_2 = last_number(line);
        sum2 += format!("{}{}", first_2.to_string(), second_2.to_string()).parse::<u64>().unwrap();
        //println!("{line} -> {first}:{second} -> {sum}");
    }
    println!("part one: final sum is {sum}");
    println!("part two: final sum is {sum2}");
}

const BASIC: [(&'static str, u32); 10] = [
    ("zero", 0), 
    ("one", 1), 
    ("two", 2), 
    ("three", 3), 
    ("four", 4), 
    ("five", 5), 
    ("six", 6), 
    ("seven", 7), 
    ("eight", 8), 
    ("nine", 9)
];

fn last_number(s: &str) -> u64 {
    let mut s = String::from(s);
    for i in 0..s.len() {
        if s.chars().last().unwrap().is_ascii_digit() {
            return s.chars().last().unwrap().to_digit(10).unwrap() as u64
        }
        if let Some(num) = BASIC.iter().find_map(|(b, i)| {
            if s.ends_with(b) { Some(i)} else { None}
        }) {
            return num.to_owned() as u64
        }
        
        s.pop();
    }
    panic!("no number found");
    
}
fn first_number(s: &str) -> u64 {
    let mut s = String::from(s);
    for i in 0..s.len() {
        if s.chars().nth(0).unwrap().is_ascii_digit() {
            return s.chars().nth(0).unwrap().to_digit(10).unwrap() as u64
        }
        if let Some(num) = BASIC.iter().find_map(|(b, i)| {
            if s.starts_with(b) { Some(i)} else { None}
        }) {
            return num.to_owned() as u64
        }
        
        s.remove(0);
    }
    panic!("no number found");
    
}