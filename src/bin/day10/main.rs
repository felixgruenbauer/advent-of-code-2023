use std::ops::Not;
use itertools::Itertools;

fn main() {
    let input = std::include_str!("input");
    let mut pipes: Vec<(usize, usize)> = vec![];
    let mut start = input.lines().enumerate().find_map(|(y, line)| {
        if let Some(x) = line.chars().position(|c| c == 'S') {
            Some((x, y))
        } else { None }
    }).unwrap();
    println!("start is at {:?}", start);
    
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    

    let mut next = {
        if ['L', '|', 'J'].contains(&grid[start.1 + 1][start.0]) {
            (start.0, start.1 + 1)
        } else if ['F', '|', '7'].contains(&grid[start.1 - 1][start.0]) {
            (start.0, start.1 - 1)
        } else if ['F', '-', 'L'].contains(&grid[start.1][start.0 - 1]) {
            (start.0 - 1, start.1)
        } else if ['J', '|', '7'].contains(&grid[start.1][start.0 + 1]) {
            (start.0 + 1, start.1)
        } else {panic!("error could not determine start pipe!")}
    };
    pipes.push(start);
    
    while next != start {
        //println!("next is {:?} {:?}", next, grid[next.1][next.0]);
        let prev = pipes.last().unwrap().clone();
        pipes.push(next);
        next = match grid[next.1][next.0] {
            '|' => {(next.0, next.1.saturating_add_signed(next.1 as isize - prev.1 as isize))},
            '-' => {(next.0.saturating_add_signed(next.0 as isize - prev.0 as isize), next.1)},
            'L' => {(next.0.saturating_add_signed(next.1 as isize - prev.1 as isize), next.1.saturating_add_signed(next.0 as isize - prev.0 as isize))},
            'J' => {(next.0 - (next.1 - prev.1), next.1 - (next.0 - prev.0))},
            'F' => {(next.0 + (next.1.abs_diff(prev.1)), next.1 + (next.0.abs_diff(prev.0)))},
            '7' => {(next.0.saturating_add_signed(next.1 as isize - prev.1 as isize), next.1.saturating_add_signed(next.0 as isize - prev.0 as isize))},
            _ => {panic!("pipe error")},
        };
        
    }
    //stops.iter().for_each(|stop| println!("{:?}", stop));
    println!("stops {:?}", pipes.len() / 2);
    
    //let mut inside = vec![];
    //let mut outside = vec![];
    //for y in 1..grid.len() {
    //    for x in 1..grid[y].len() {
    //        if is_outside((x, y), grid[0].len() - 1, grid.len() - 1, &stops) {
    //            outside.push((x, y));
    //            continue
    //        }
    //        //{
    //        //    outside.push((x, y));
    //        //    continue;
    //        //} else if outside.

    //        
    //    }


    //}
    //outside.iter().for_each(|t| println!("{:?}", t));
    //println!("{} {}", outside.len(), grid.len() * grid[0].len());
    
    pipes.insert(0, pipes.last().unwrap().clone());
    pipes.push(pipes.first().unwrap().clone());
    let lines = pipes.iter().enumerate().skip(1usize).rev().skip(1).rev()
        .filter(|(idx, pipe)| pipes[idx - 1].1 == pipe.1 - 1 || pipes[idx + 1].1 == pipe.1 - 1)
        .map(|(_, pipe)| pipe)
        .sorted_by_key(|pipe| pipe.0)
        .into_group_map_by(|pipe| pipe.1);
    lines.iter().for_each(|l| println!("{:?}", l));
    
    let sum: usize = lines.into_iter().map(|(idx, line)| {
        line.iter().tuples().map(|(a, b)| (a.0..b.0).filter(|&x| !pipes.contains(&(x, idx))).count()).sum::<usize>()
    }).sum();
    println!("sum: {}", sum);

}

fn is_outside(tile: (usize, usize), max_x: usize, max_y: usize, stops: &Vec<(usize, usize)>) -> bool {
    let (x, y) = tile;
    if x == 0usize || y == 0usize || x == max_x || y == max_y {
        return true
    }
    let mut to_check = (1usize..=3).flat_map(|x| (1usize..=3).map(move |y: usize| (tile.0 + x - 2, tile.1 + y - 2))).filter(|t| stops.contains(t).not() || t != &tile).peekable();
    if to_check.peek().is_none() {
        return false
    }
    return to_check.any(|t| is_outside(t, max_x, max_y, stops))
    
}
