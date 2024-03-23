use std::str::FromStr;
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = std::include_str!("input");
    let mut lines = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>();
    let sum: usize = lines.iter().map(|problem| {
        let mut intermediate_results: HashMap<(Vec::<State>, Vec::<usize>), usize> = HashMap::new();
        let res = arrangements_improved(&[problem.0.clone(), vec![State::Working]].concat(), &problem.1, &mut intermediate_results);
        println!{"{:?}: {:?} -> {}", problem.0, problem.1, res};
        res
    }).sum();
    println!("part one, sum: {}", sum);

    
    
    let sum = lines
        .par_iter_mut()
        .enumerate()
        .map(|(line, (record, groups))| {
            let mut record = [record.as_slice(); 5].join(&State::Unknown);
            record.push(State::Working);
            let groups = groups.as_slice().repeat(5);
            let mut intermediate_results: HashMap<(Vec::<State>, Vec::<usize>), usize> = HashMap::new();
            let res = arrangements_improved(&record, &groups, &mut intermediate_results);
            println!("{line} -> {res}");
            res
        })
        .sum::<usize>();
    println!("part two: {}", sum);

 
}

fn arrangements(record: &[State], groups: &[usize]) -> usize {
    println!("{:?} :: {:?}", record, groups);
    let rec_len = record.len();
    if rec_len < groups.len() + groups.iter().sum::<usize>() {return 0}
    match (record, groups) {
        (_, []) if !record.contains(&State::Damaged) => 1,
        ([State::Damaged, _], [group, rest @ ..]) if *group < rec_len => {
            if !record[0..*group].contains(&State::Working)
                && record[*group] != State::Damaged {
                    arrangements(&record[*group+1..], rest)
            } else {0}
        },
        ([State::Unknown, record_rest @ ..], [group, rest @ ..]) if *group < rec_len => {
            if !record[0..*group].contains(&State::Working) {
                match record[*group] {    
                    State::Working if record[0..*group].contains(&State::Damaged) => arrangements(&record[*group+1..], rest),
                    State::Working => arrangements(&record[*group+1..], rest) + arrangements(&record[*group+1..], groups),
                    State::Unknown => arrangements(&record[*group+1..], rest) + arrangements(record_rest, groups),
                    State::Damaged => arrangements(record_rest, groups)
                }
            } else {
                arrangements(record_rest, groups)
            }
        },
        ([State::Working, record_rest @ ..], _) => {
            arrangements(record_rest, groups)
        },
        _ => 0,
    }
}

fn arrangements_improved(record: &[State], groups: &[usize], intermediate_results: &mut HashMap<(Vec::<State>, Vec::<usize>), usize>) -> usize {
    if let Some(res) = intermediate_results.get(&(record.to_vec(), groups.to_vec())) {
        return *res
    };
    let rec_len = record.len();
    if rec_len < groups.len() + groups.iter().sum::<usize>() {return 0}
    let res = match (record, groups) {
        (_, []) if !record.contains(&State::Damaged) => 1,
        ([State::Damaged, _], [group, rest @ ..]) if *group < rec_len => {
            if !record[0..*group].contains(&State::Working)
                && record[*group] != State::Damaged {
                    arrangements_improved(&record[*group+1..], rest, intermediate_results)
            } else {0}
        },
        ([State::Unknown, record_rest @ ..], [group, rest @ ..]) if *group < rec_len => {
            if !record[0..*group].contains(&State::Working) {
                match record[*group] {    
                    State::Working if record[0..*group].contains(&State::Damaged) => arrangements_improved(&record[*group+1..], rest, intermediate_results),
                    State::Working => arrangements_improved(&record[*group+1..], rest, intermediate_results) + arrangements_improved(&record[*group+1..], groups, intermediate_results),
                    State::Unknown => arrangements_improved(&record[*group+1..], rest, intermediate_results) + arrangements_improved(record_rest, groups, intermediate_results),
                    State::Damaged => arrangements_improved(record_rest, groups, intermediate_results)
                }
            } else {
                arrangements_improved(record_rest, groups, intermediate_results)
            }
        },
        ([State::Working, record_rest @ ..], _) => {
            arrangements_improved(record_rest, groups, intermediate_results)
        },
        _ => 0,
    };
    intermediate_results.insert((record.to_vec(), groups.to_vec()), res);
    res
}

fn parse_line(line: &str) -> (Vec<State>, Vec<usize>) {
    let record = line.split_whitespace().next().unwrap().chars().map(|c| c.to_string().parse::<State>().unwrap()).collect::<Vec<_>>();
    let groups: Vec<usize> = line.split_whitespace().last().unwrap().split(",").map(|n| n.parse::<usize>().unwrap()).collect();
    (record, groups)  
}


#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
enum State {
    Unknown,
    Damaged,
    Working     
}

#[derive(Debug, PartialEq, Eq)]
struct ParseStateError;

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(State::Working),
            "#" => Ok(State::Damaged),
            "?" => Ok(State::Unknown),
            _ => Err(ParseStateError) 
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_arrangements_improved() {
        let sum = TEST_INPUT.lines().zip(TEST_SOLUTION_1.iter()).map(|(line, res)| {
            let (mut record, groups) = parse_line(line);
            record.push(State::Working);
            let mut intermediate_results = HashMap::new();
            //let test_res = arrangements(&record, &groups);
            let test_res = arrangements_improved(&record, &groups, &mut intermediate_results);
            assert_eq!(test_res, *res);
            test_res
        }).sum::<usize>();
        assert_eq!(sum, TEST_SOLUTION_1.iter().sum());
    }

    #[test]
    fn test_expanded_arrangements_improved() {
        let sum = TEST_INPUT.lines().zip(TEST_SOLUTION_2.iter()).map(|(line, res)| {
            let (record, groups) = parse_line(line);
            let mut record = [record.as_slice(); 5].join(&State::Unknown);
            record.push(State::Working);
            let groups = groups.as_slice().repeat(5);
            let mut intermediate_results = HashMap::new();
            let test_res = arrangements_improved(&record, &groups, &mut intermediate_results);
            //let test_res = arrangements(&record, &groups);
            assert_eq!(test_res, *res);
            test_res
        }).sum::<usize>();
        assert_eq!(sum, TEST_SOLUTION_2.iter().sum());
    }
}

const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
const TEST_SOLUTION_1: [usize; 6] = [1,4,1,1,4,10];
const TEST_SOLUTION_2: [usize; 6] = [1,16384,1,16,2500,506250];