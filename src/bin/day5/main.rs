use std::{clone, cmp::min, collections::{BTreeMap, HashMap}};

fn main() {
    let input = std::include_str!("input");
    //let input = TEST_INPUT;
    let almanac = parse_input(input);
    println!("{:?}", almanac.mappings.keys().collect::<Vec<_>>());
    //println!("{ORDER:?}");
    //for seed in almanac.seeds.iter() {
    //    let location = ORDER.iter().fold(seed.to_owned(), |acc, m| calc_output(&acc, &almanac.mappings[m]));
    //    println!("{seed} => {location}");
    //}
    let (seed, location) = almanac.seeds.iter().map(|s| (s, ORDER.iter().fold(s.to_owned(), |acc, m| calc_output(&acc, &almanac.mappings[m])))).reduce(|acc, s| if s.1 < acc.1 { s } else { acc }).unwrap();
    println!("seed with lowest location: {seed} => {location}");

    //let (seed, location) = almanac.seeds.iter().step_by(2).zip(almanac.seeds.iter().skip(1).step_by(2)).flat_map(|(start, end)| start.to_owned()..start+end).map(|s| (s, ORDER.iter().fold(s.to_owned(), |acc, m| calc_output(&acc, &almanac.mappings[m])))).reduce(|acc, s| if s.1 < acc.1 { s } else { acc }).unwrap();
    //println!("seed with lowest location: {seed} => {location}");

    let res = lowest_location(&almanac);
    println!("part two: lowest location is {res}");
}

const ORDER: [&str; 7] = ["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];

fn lowest_location(almanac: &Almanac) -> usize {
    let mut values: Vec<(usize, usize)> = almanac.seeds.iter().step_by(2).zip(almanac.seeds.iter().skip(1).step_by(2)).map(|(v1, v2)| (v1.clone(), v2.clone())).collect(); 
    for mapping in ORDER.iter() {
        //let ranges = almanac.mappings[mapping].clone();
        values = values.into_iter().flat_map(|value| {
            let mut res = vec![];
            let mut current = value.0.to_owned();
            let end = value.0 + value.1;
            for range in &almanac.mappings[mapping] {
                if current < range.1 {
                    if end <= range.1 {
                        res.push((current, value.1.clone()));
                        break;
                    } else {
                        res.push((current, range.1 - current));
                        current = range.1;
                    }
                }
                if current < range.1 + range.2 {
                    if end <= range.1 + range.2 {
                        res.push((current.saturating_add_signed(range.0 as isize - range.1 as isize), end - current));
                        break;
                    } else {
                        
                        res.push((current.saturating_add_signed(range.0 as isize - range.1 as isize), range.1 + range.2 - current));
                        current = range.1 + range.2;
                    }
                }
            }
            res
        }).collect()


    }
    println!("{values:?}");
    values.iter().map(|v| v.0).min().unwrap()

}

#[derive(Debug)]
struct Almanac<'a> {
    mappings: BTreeMap<&'a str, Vec<Range>>,
    seeds: Vec<usize>
}

#[derive(Debug, Clone, Copy)]
struct Range (usize, usize, usize);

fn parse_input(s: &str) -> Almanac {
    let mut line_iter = s.lines();
    let seeds: Vec<usize> = line_iter.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<usize>().unwrap()).collect();
    let mut mappings = BTreeMap::new();
    let mut ranges = vec![];
    let mut name: &str = "";
    for line in line_iter {
        if let Some(first) = line.chars().next() {
            if first.is_ascii_alphabetic() {
                name = line.split_whitespace().next().unwrap();
            } else if first.is_ascii_digit() {
                let range: Vec<_>= line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
                ranges.push(Range(range[0], range[1], range[2]));
            }
        } else if name.len() != 0 {
            ranges.sort_by_key(|r| r.1);
            mappings.insert(name, ranges.to_owned());
            ranges.clear();
        }
    }
    
    if name.len() != 0 && !ranges.is_empty() {
        ranges.sort_by_key(|r| r.1);
        mappings.insert(name, ranges.to_owned());
    }

    Almanac { mappings, seeds }
}

fn calc_output(input: &usize, ranges: &Vec<Range>) -> usize {
    let mut output = input.to_owned();
    for range in ranges.iter(){
        if (range.1..range.1+range.2).contains(&input) {
            let offset: isize = range.0 as isize - range.1 as isize;
            //return input.saturating_add_signed(offset)
            output = input.saturating_add_signed(offset);
            break;
        }

    }
    //println!("{ranges:?}");
    //println!("{input}");
    //println!("{output}");
    output
    //input.to_owned()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calc_output() {
        assert_eq!(calc_output(&33, &vec![Range(20, 30, 4)]), 23);
        assert_eq!(calc_output(&33, &vec![Range(20, 30, 3)]), 33);
        assert_eq!(calc_output(&30, &vec![Range(20, 30, 3)]), 20);
        assert_eq!(calc_output(&13, &vec![Range(20, 10, 3)]), 13);
        assert_eq!(calc_output(&12, &vec![Range(20, 10, 3)]), 22);
        assert_eq!(calc_output(&11, &vec![Range(20, 10, 3)]), 21);
        assert_eq!(calc_output(&10, &vec![Range(20, 10, 3)]), 20);
        assert_eq!(calc_output(&3, &vec![Range(20, 10, 3)]), 3);
    }
    
    #[test]
    fn test_parse_input() {
        println!("{:?}", parse_input(TEST_INPUT));
    }
}
const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";