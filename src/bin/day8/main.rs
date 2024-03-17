use std::collections::HashMap;

fn main() {
    let mut line_iter = std::include_str!("input").lines();
    let instructions = line_iter.next().unwrap();
    let nodes: HashMap<&str, (&str, &str)> = line_iter.skip(1).map(|line| {
        unsafe {
            (line.get_unchecked(0..3), (line.get_unchecked(7..10), line.get_unchecked(12..15)))
        }
    }).collect();

    let mut currents: Vec<_> = nodes.keys().filter(|k| k.ends_with("A")).collect();
    let mut res = vec![];
    for current in currents.into_iter() {
        let mut temp = current;
        for (step, ins) in instructions.chars().cycle().enumerate() {
            if ins == 'R' {
                temp = &nodes[temp].1
            } else {
                temp = &nodes[temp].0
            }
            if temp.ends_with("Z") { 
                res.push(step + 1);
                break;
            }
        }
    }
    println!("{:?}", res.iter().copied().reduce(|acc, n| lcm(acc, n)).unwrap());
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}


fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					    