use core::panic;

use structopt::StructOpt;

const DAYS_INPUT: [&str; 3] = ["1", "2", "3"];
const PART_INPUT: [&str; 2] = ["1", "2"];

#[derive(StructOpt, Debug)]
#[structopt(name = "Advent of Code")]
struct Opt {
    #[structopt(short,long,possible_values = &DAYS_INPUT,help = r"Day to run")]
    day: usize,
    #[structopt(short,long,possible_values = &PART_INPUT,help = r"Part to run")]
    part: usize,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let answer = match (opt.day, opt.part) {
        (1, p) => day01::run(p).await,
        (2, p) => day02::run(p).await,
        (_, _) => panic!("SHIT"),
    };

    println!("Day {} Part {}: {}", opt.day, opt.part, answer)
}
