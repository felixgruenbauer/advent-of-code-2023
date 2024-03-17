use core::fmt;

fn main() {
    let content = std::include_str!("input");
    let mut games = vec![];
    for line in content.lines() {
        let game = parse_line(line).unwrap();
        println!("{line}\n=> {game}");
        games.push(game);        
    }

    // only 12 red cubes, 13 green cubes, and 14 blue cubes.
    let sum = games.iter().filter(|g| g.subsets.iter().all(|s| s.red < 13 && s.green < 14 && s.blue < 15)).fold(0u32, |acc, g| acc + g.num);
    println!("first part: total sum is {sum}");

    let mut sum = 0u32;
    for game in games.iter() {
        let (mut red, mut green, mut blue) = (0u32, 0u32, 0u32);
        game.subsets.iter().for_each(|s| { red = s.red.max(red); blue = s.blue.max(blue); green = s.green.max(green);});
        sum += red * green * blue;
    }
    println!("second part: sum of powers is {sum}");
}

#[derive(Debug)]
struct Game {
    num: u32,
    subsets: Vec<Subset>
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!("Game {}\n", self.num);
        for subset in self.subsets.iter() {
            output.push_str(format!("   red: {}, blue: {}, green: {}\n", subset.red, subset.blue, subset.green).as_str());
        }
        
        write!(f, "{}", output)
    }
}

#[derive(Debug, Default)]
struct Subset {
    red: u32,
    blue: u32,
    green: u32
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
fn parse_line(s: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let i: Vec<&str> = s.split(":").collect();
    let subsets = parse_subsets(i[1]);    
    Ok(Game { num: i[0].split(" ").last().unwrap().parse::<u32>().unwrap(), subsets})
}

fn parse_subsets(s: &str) -> Vec<Subset> {
    let mut subsets = vec![];
    for set in s.split(";") {
        let mut subset = Subset::default();
        for color in set.split(",") {
            let mut iter = color.trim_start_matches(" ").split(" ");
            let count = iter.next().unwrap().parse::<u32>().unwrap();
            match iter.next().unwrap() {
                "green" => subset.green = count,
                "blue" => subset.blue = count,
                "red" => subset.red = count,
                _ => {}
            } 
            
        }
        subsets.push(subset);
    }
    subsets

}