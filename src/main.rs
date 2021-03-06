mod solutions;
mod util;

use solutions::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    let start = std::time::Instant::now();
    match problem {
        "day_1" => day_1::all(),
        "day_1_a" => day_1::day_1_a(),
        "day_1_b" => day_1::day_1_b(),
        "day_2" => day_2::all(),
        "day_2_a" => day_2::day_2_a(),
        "day_2_b" => day_2::day_2_b(),
        "day_3" => day_3::all(),
        "day_3_a" => day_3::day_3_a(),
        "day_3_b" => day_3::day_3_b(),
        "day_4" => day_4::all(),
        "day_4_a" => day_4::day_4_a(),
        "day_4_b" => day_4::day_4_b(),
        "day_5" => day_5::all(),
        "day_5_a" => day_5::day_5_a(),
        "day_5_b" => day_5::day_5_b(),
        "day_6" => day_6::all(),
        "day_6_a" => day_6::day_6_a(),
        "day_6_b" => day_6::day_6_b(),
        "day_7" => day_7::all(),
        "day_7_a" => day_7::day_7_a(),
        "day_7_b" => day_7::day_7_b(),
        "day_8" => day_8::all(),
        "day_8_a" => day_8::day_8_a(),
        "day_8_b" => day_8::day_8_b(),
        "all" => {
            day_1::all();
            day_2::all();
            day_3::all();
            day_4::all();
            day_5::all();
            day_6::all();
            day_7::all();
            day_8::all();
        }
        _ => println!("Solution not ready for problem: {}", problem),
    }
    println!("Elapsed time: {:?}.", start.elapsed());
}
