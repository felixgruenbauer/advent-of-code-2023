use std::{iter::StepBy, str::FromStr};

fn main() {
    let mut problems = std::include_str!("input").lines().map(|line| {
        let mut record = line.split_whitespace().next().unwrap().chars().map(|c| c.to_string().parse::<State>().unwrap()).collect::<Vec<_>>();
        record.extend(vec![record.clone(); 4].iter_mut().map(|r| {r.insert(0, State::Unknown); r.clone()}).flatten());
        // adding . at the end of every record adds a group end without changing th result
        record.push(State::Working);
        let mut groups: Vec<usize> = line.split_whitespace().last().unwrap().split(",").map(|n| n.parse::<usize>().unwrap()).collect();
        groups.extend(vec![groups.clone(); 4].iter().flatten());
        (record, groups)
    }).collect::<Vec<_>>();
 
    let sum: usize = problems.iter().map(|problem| {
        let res = arrangements(&problem.0, &problem.1);
        //println!("{:?} -> {:?}", problem, res);
        res
    }).sum();
    println!("part one, sum: {}", sum);
}

fn arrangements(record: &[State], groups: &[usize]) -> usize {
    //println!("{:?}    {:?}", record, groups);
    match (record, groups) {
        ([], []) => 1,
        ([State::Unknown | State::Working, rest @ ..], []) if record.iter().all(|s| matches!(s, State::Unknown | State::Working)) => 1,
        ([State::Damaged, record_rest @ ..], [group, rest @ ..]) if *group < record.len() => {
            if record.iter().take(*group).all(|s| matches!(s, State::Damaged | State::Unknown)) 
                && matches!(record.iter().skip(*group).next().unwrap(), State::Working | State::Unknown) { 
                    arrangements(record.iter().skip(group + 1).cloned().collect::<Vec<_>>().as_slice(), rest)
            } else {0}
        },
        ([State::Unknown, record_rest @ ..], [group, rest @ ..]) if *group < record.len() => {
            //println!("{:?}  {:?}", record, groups);
            if record.iter().take(*group).all(|s| matches!(s, State::Damaged | State::Unknown)) {
                match record.iter().skip(*group).next().unwrap() {    
                    State::Working if record.iter().take(*group).find(|&s| s == &State::Damaged).is_some() => arrangements(record.iter().skip(*group + 1).cloned().collect::<Vec<_>>().as_slice(), rest),
                    State::Working => arrangements(record.iter().skip(*group + 1).cloned().collect::<Vec<_>>().as_slice(), rest) + arrangements(record.iter().skip(*group + 1).cloned().collect::<Vec<_>>().as_slice(), groups),
                    State::Unknown => arrangements(record.iter().skip(*group + 1).cloned().collect::<Vec<_>>().as_slice(), rest) + arrangements(record_rest, groups),
                    State::Damaged => arrangements(record_rest, groups)
                }
            } else {
                arrangements(record_rest, groups)
            }
        },
        ([State::Working, record_rest @ ..], _) => {
            arrangements(record_rest, groups)
        },
        _ => {
            //println!("{:?}  {:?}", record, groups);
            0
        },
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
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