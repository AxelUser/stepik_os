use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read tasks count");
    let count = input.trim().parse::<usize>().expect("Failed to parse tasks count");

    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read tasks duration");
    
    
    let mut dur_sorted = input.trim().splitn(count, " ").map(|d| d.parse::<u32>().expect("Failed to parse duration"))
        .enumerate().collect::<Vec<(usize, u32)>>();
    dur_sorted.sort_by(|(_, a), (_, b)| a.cmp(b));
    dur_sorted.iter().for_each(|(i, _)| print!("{0} ", i));
        
    std::io::stdout().flush().expect("Failed to flush");
}
