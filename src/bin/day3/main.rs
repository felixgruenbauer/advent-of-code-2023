// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..

use std::{cmp::min, collections::HashMap};



fn main() {
    let input = std::include_str!("input");
    let lines = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let numbers = find_numbers(&lines).expect("error parsing numbers!");
    let pot_engine_numbers = check_adjacent(&lines, &numbers, |x| x == '*');
    let sum2 = calc_engine_power(&pot_engine_numbers);
    let part_numbers = check_adjacent(&lines, &numbers, |x| !x.is_ascii_alphanumeric() && x !='.');
    let sum = part_numbers.iter().fold(0u32, |acc, num| acc + num.0.0);
    println!("part one: sum is {sum}");
    println!("part two: sum is {sum2}");
    
}

fn calc_engine_power(numbers: &Vec<(Number, Coordinates)>) -> u32 {
    let mut engine_numbers = HashMap::new();
    for number in numbers.iter() {
        for number2 in numbers.iter() {
            if number == number2 { continue; }
            if number.1 == number2.1 {
                engine_numbers.insert(number.1, number.0.0 * number2.0.0);
            }
        }
    }
    let mut sorted: Vec<_> = engine_numbers.iter().collect();
    sorted.sort_by_key(|a| a.0);
    engine_numbers.iter().fold(0u32, |acc, (_, n2)| acc + n2)

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Number(u32, usize, usize, usize);

fn find_numbers(lines: &Vec<Vec<char>>) -> Result<Vec<Number>, Box<dyn std::error::Error>> {
    let mut numbers = vec![];
    
    for (line_id, line) in lines.iter().enumerate() {
        let mut num: String = "".to_string();
        for (cell_id, symbol) in line.iter().enumerate() {
            if symbol.is_ascii_digit() {
                num.push(*symbol);
                if cell_id == lines[0].len() - 1 {
                    numbers.push(Number {0: num.parse::<u32>()?, 1: line_id, 2: cell_id - num.len() + 1, 3: num.len()});
                }
            } else if num.len() > 0 {
                numbers.push(Number {0: num.parse::<u32>()?, 1: line_id, 2: cell_id - num.len(), 3: num.len()});
                num = String::from("");
            }
        }
    }
    Ok(numbers)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coordinates(usize, usize);

fn check_adjacent<F>(lines: &Vec<Vec<char>>, numbers: &Vec<Number>, check_fn: F) -> Vec<(Number, Coordinates)>
where
    F: Fn(char) -> bool
{
    let mut res = vec![];
    for number in numbers.iter() {
        let Number(_, line_id, start, len) = number.to_owned();
        for line in line_id.saturating_sub(1)..=min(lines[0].len() - 1, line_id+1) {
            for cell in start.saturating_sub(1)..=min(lines[0].len() - 1, start+len) {
                if check_fn(lines[line][cell]) {
                    res.push((number.to_owned(), Coordinates(line, cell)));
                    break;
                }
            }
        }
    }
    res
}