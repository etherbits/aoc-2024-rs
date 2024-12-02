use std::{env::args, process::exit};
mod d1;
mod utils;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        println!("cargo run {{day}} {{part}}");
        exit(1);
    }

    let q = args[1].clone() + "-" + &args[2];

    match q.as_str() {
        "1-1" => {
            println!("Running d1 p1");
            d1::p1()
        }
        "1-2" => {
            println!("Running d1 p2");
            d1::p2()
        }
        _ => println!("No such solution"),
    }
}
