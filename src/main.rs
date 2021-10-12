use std::env;
use console::style;
use rand::{prelude::SliceRandom, thread_rng};

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    println!("{} {}",
             style("Your random string:").red(),
             args.choose(&mut thread_rng()).unwrap_or(&"No arguments".to_string())
             );
}
