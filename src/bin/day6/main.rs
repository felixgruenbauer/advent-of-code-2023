use std::ops::{Div, DivAssign};

const INPUT: &str = "Time:        42     89     91     89
Distance:   308   1170   1291   1467";
fn main() {
    let races: Vec<_> = INPUT.lines().next().unwrap().split_ascii_whitespace()
        .skip(1).zip(INPUT.lines().last().unwrap().split_ascii_whitespace().skip(1))
        .map(|(t, d)| (t.parse::<usize>().unwrap(), d.parse::<usize>().unwrap())).collect();
    let mut res = 1usize;
    for (time, dist) in races.iter() {
        for i in 1usize..time.div_ceil(2) {
            if i * (time - i) > *dist {
                let temp = time - i * 2 + 1;
                res = res * temp;
                println!("{}, {} -> {} * {} -> {}", time, dist, i, time - i, temp);
                break;
            }
        }
    }
    println!("result: {res}");

    let race = races.iter().fold(("".to_string(), "".to_string()), |acc, (t, d)| (acc.0+&t.to_string(), acc.1+&d.to_string()));
    let mut res = 1usize;
    let (time, dist) = (race.0.parse::<usize>().unwrap(), race.1.parse::<usize>().unwrap());
    for i in 1usize..time.div_ceil(2) {
        if i * (time - i) > dist {
            let temp = time - i * 2 + 1;
            res = res * temp;
            println!("{}, {} -> {} * {} -> {}", time, dist, i, time - i, temp);
            break;
        }
    }
    println!("result part two: {res}");

}
