fn main() {
    let lines: Vec<_> = std::include_str!("input").lines().map(|line| line.split_whitespace().map(|c| c.parse::<isize>().unwrap()).collect::<Vec<_>>()).collect();
    let res = lines.iter().fold(0isize, |acc, i| acc + last_value(&i));
    println!("part one: sum is {}", res);

    let res = lines.iter().fold(0isize, |acc, i| acc + first_value(&i));
    println!("part two: sum is {}", res);
}

fn last_value(input: &Vec<isize>) -> isize {
    if input.iter().all(|i| i == &0) {
        return 0
    } else {
        return input.last().unwrap() + last_value(&input.iter().enumerate().skip(1).map(|(idx, n)| n - input[idx - 1]).collect())
    }
}

fn first_value(input: &Vec<isize>) -> isize {
    if input.iter().all(|i| i == &0) {
        return 0
    } else {
        return input[0] - first_value(&input.iter().enumerate().skip(1).map(|(idx, n)| n - input[idx - 1]).collect()) 
    }
}
