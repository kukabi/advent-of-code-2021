mod solutions;
mod util;

use solutions::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    match problem {
        "day_1_a" => day_1::day_1_a(),
        "day_1_b" => day_1::day_1_b(),
        "day_2_a" => day_2::day_2_a(),
        "day_2_b" => day_2::day_2_b(),
        "day_3_a" => day_3::day_3_a(),
        _ => println!("Solution not ready for problem: {}", problem),
    }
}
