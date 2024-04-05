use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};


fn main() {
    let input = std::include_str!("input");

    println!("part one: {}", solve1(&parse_columns(input)));
   
    println!("part two: {}", solve2(input));
}
    

fn solve2(input: &str) -> usize {
    let mut columns = parse_columns(input);
    let mut cache = Vec::new();
    let cycles = 1000000000usize;
    for i in 0..cycles {
        cache.push(weight(&columns));
        for win_len in 3..(i/2).min(100) {
            if cache[cache.len() - win_len..] == cache[cache.len()  - 2* win_len..cache.len() - win_len] {
                let pos = ((cycles - (i - win_len)) % (win_len)) + (i -  win_len);
                println!("{:?}", cache);
                println!("cycle {}, win_len {}, pos {}", i, win_len, pos);
                return cache[pos]
            }
        }
        columns = cycle(&mut columns);
       
    }
    weight(&columns)

}

fn parse_columns(input: &str) -> Vec<Vec<char>> {
    let res = (0..input.lines().next().unwrap().len()).map(|col_id| {
        input.lines().map(|line| line.chars().nth(col_id).unwrap()).collect::<Vec<char>>()
    }).collect::<Vec<_>>();
    res
}

fn cycle(columns: &mut Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut columns = columns.clone();
    for _ in 0..4 {
        tilt(&mut columns);
        columns = rotate(&mut columns);
    }
    columns
}

fn solve1(columns: &Vec<Vec<char>>) -> usize {
    columns.iter().map(|col| {
        let mut last_rock = 0usize;
        col.iter().enumerate().map(|(idx, c)| {
            match c {
                '#' => {
                    last_rock = idx + 1;
                    0
                },
                'O' => {
                    let res = col.len() - last_rock;
                    last_rock += 1;
                    res
                },
                _ => 0
            }
        }).sum::<usize>()
    }).sum::<usize>()
}

fn weight(columns: &Vec<Vec<char>>) -> usize {
    columns.iter().map(|col| {
        col.iter().rev().enumerate().map(|(idx, c)| {
            if c == &'O' {
                idx + 1
            } else {0}
        }).sum::<usize>()
    }).sum::<usize>()
}
fn rotate(columns: &mut Vec<Vec<char>>) -> Vec<Vec<char>>{
    (0..columns[0].len()).rev().map(|line| columns.iter().map(|col| col[line]).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn tilt(columns: &mut Vec<Vec<char>>) {
    columns.par_iter_mut().for_each(|col| {
        let mut last_rock = 0usize;
        for cell in 0..col.len() {
            match col[cell] {
                '#' => {
                    last_rock = cell + 1;
                },
                'O' => {
                    col.swap(cell, last_rock);
                    last_rock += 1;
                },
                _ => {}
            }
        }
    })
}


fn _pprint_columns(columns: &Vec<Vec<char>>) {
    for line in 0..columns[0].len() {
        columns.iter().for_each(|col| print!("{}", col[line]));
        print!("\n");
    }
    println!("\n");
}


#[cfg(test)]
mod tests {
    use crate::{cycle, parse_columns, solve2, tilt, weight};

    #[test]
    fn test_weight() {
        let input = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        assert_eq!(weight(&parse_columns(input)), 87);

    }
    #[test]
    fn test_solve2() {

        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let res = solve2(input);
        assert_eq!(res, 64);
    }

    #[test]
    fn test_cycle() {

        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        
        let output = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

        let output = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        let a = cycle(&mut cycle(&mut cycle(&mut parse_columns(input))));
        let b = parse_columns(output);
        assert_eq!(a, b);
    }
    
    #[test]
    fn test_tilt() {

        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        
        let output = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        
        let mut columns = parse_columns(input);
        tilt(&mut columns);
        assert_eq!(columns, parse_columns(output))
    }
}