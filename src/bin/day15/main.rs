fn main() {
    let input = std::include_str!("input").replace("\n", "");
    println!("part one: {}", solve1(&input));
    println!("part two: {}", solve2(&input));

}

fn solve1(input: &str) -> usize {
    input.split(",").map(|step| 
        step.chars()
            .fold(0usize, |acc, c| ((acc + c as usize) * 17) % 256)
    ).sum()
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0usize, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn solve2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for step in input.split(",") {
        if step.contains(&"-") {
            let label = &step[..step.len() - 1];
            let boxx = &mut boxes[hash(&label)];
            boxx.retain(|lens| lens.0 != label);
        } else {
            let label = &step[..step.len() - 2];
            let boxx = &mut boxes[hash(&label)];
            let power = step.chars().last().unwrap().to_digit(10).unwrap() as usize;
            if let Some(lens) = boxx.iter_mut().find(|(l, _)| *l == label) {
                lens.1 = power;
            } else {
                boxx.push((label, power));
            }
        }
    }
    
    boxes.iter().enumerate()
        .map(|(idx, boxx)| boxx.iter().enumerate()
            .map(|(l_idx, (_, power))| (idx + 1) * (l_idx + 1) * power)
            .sum::<usize>()
        ).sum()
}