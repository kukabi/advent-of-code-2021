mod solutions;
mod util;

use solutions::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    match problem {
        "day_1_a" => day_1::day_1_a(),
        "day_1_b" => day_1::day_1_b(),
        _ => println!("Unknown problem: {}", problem),
    }
}
