use std::usize;



fn main() {
    
    let patterns: Vec<Vec<Vec<char>>> = std::include_str!("input").split("\n\n").map(|pattern| pattern.lines().map(|line| line.chars().collect()).collect()).collect();
    
    let sum =  patterns.iter().map(|pattern| solve_pattern(pattern)).sum::<usize>();
    println!("part one: {}", sum);
    let sum =  patterns.iter().map(|pattern| solve_pattern2(pattern)).sum::<usize>();
    println!("part one: {}", sum);
}


fn solve_pattern(pattern: &Vec<Vec<char>>) -> usize {
    let line_len = pattern[0].len();
    for col in 1..line_len {
        let max_ref_len = col.min(line_len - col);
        if pattern.iter().all(|line| {
            line[col - max_ref_len..col] == line[col..col + max_ref_len].iter().rev().cloned().collect::<Vec<_>>()
        })
        {
            return col
        }
    }
    
    let col_len = pattern.len();
    for ref_line in 1..col_len {
        let max_ref_dist = ref_line.min(col_len - ref_line);
        if (0usize..max_ref_dist).all(|dist| {
            pattern[ref_line + dist] == pattern[ref_line - dist - 1]
        })
        {
            return ref_line * 100
        }
    }
    panic!("no reflection found!");

}

fn solve_pattern2(pattern: &Vec<Vec<char>>) -> usize {
    let line_len = pattern[0].len();
    for col in 1..line_len {
        let max_ref_len = col.min(line_len - col);
        if pattern.iter().map(|line| {
            line[col - max_ref_len..col].iter().zip(line[col..col + max_ref_len].iter().rev()).map(|(a, b)| usize::from(a != b)).sum::<usize>()
        }).sum::<usize>() == 1
        {
            return col
        }
    }
    
    let col_len = pattern.len();
    for ref_line in 1..col_len {
        let max_ref_dist = ref_line.min(col_len - ref_line);
        if (0usize..max_ref_dist).map(|dist| {
            pattern[ref_line + dist].iter().zip(pattern[ref_line - dist - 1].iter()).map(|(a, b)| usize::from(a != b)).sum::<usize>()
        }).sum::<usize>() == 1
        {
            return ref_line * 100
        }
    }
    panic!("no reflection found!");
            
}