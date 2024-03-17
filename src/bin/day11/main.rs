
fn main() {
    let mut exp_rows = vec![];
    let mut universe_unexp: Vec<Vec<_>> = std::include_str!("input").lines().map(|line| line.chars().collect()).collect();
    let mut universe: Vec<Vec<_>> = std::include_str!("input").lines().enumerate()
        .flat_map(|(row, line)| {
            let chars: Vec<char> = line.chars().collect();
            //let count = if chars.iter().all(|&c| c == '.') {1000000} else {1};
            let count = if chars.iter().all(|&c| c == '.') {exp_rows.push(row); 2} else {1};
            std::iter::repeat(chars).take(count)})
        .collect();
    
    let mut exp_cols = vec![];
    for col in 0..universe_unexp[0].len() {
        if universe.iter().map(|row| row[col]).all(|c| c == '.') {
            exp_cols.push(col);
        }
    }
    
    let galaxies = universe_unexp.iter().enumerate()
        .flat_map(|(y, row)| 
            row.iter().enumerate().filter_map(move |(x, &space)| {
                if space == '#' {Some((x, y))} else {None}
            })).collect::<Vec<_>>();
    
    println!("found {} galaxies: {:?} - {:?}", galaxies.len(), galaxies.first().unwrap(), galaxies.last().unwrap());
    //galaxies.iter().for_each(|g| println!("{:?}", g));
    

    let mut sum = 0usize;
    let mut pairs = 0usize;
    //let galaxies = galaxies.get(0..10).unwrap();
    let mut expanded_paths = 0usize;
    for (idx, g_a) in galaxies.iter().enumerate() {
        for g_b in galaxies.get(idx..galaxies.len()).unwrap().iter() {
            let dist = (g_b.0.abs_diff(g_a.0)) + (g_b.1.abs_diff(g_a.1));
            sum += dist;
            
            expanded_paths += (g_b.0.min(g_a.0)..g_b.0.max(g_a.0)).filter(|space| exp_cols.contains(space)).count();
            expanded_paths += (g_b.1.min(g_a.1)..g_b.1.max(g_a.1)).filter(|space| exp_rows.contains(space)).count();

            
            //println!("{:?} x {:?} = {}", g_a, g_b, temp);
            pairs += 1;
        }
    }
    println!("sum1 is {}", sum + expanded_paths);
    println!("sum2 is {}", sum + 999999 * expanded_paths);
    println!("{} pairs", pairs);
 

}
