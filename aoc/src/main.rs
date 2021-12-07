use core::panic;
use num_enum::TryFromPrimitive;
use std::fmt::Error;
use std::str::FromStr;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
#[derive(Clone,Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Days {
    One = 1,
    Two = 2,
    }
}

fn days_parser(s: &str) -> Days {
    match u8::from_str(s) {
        Ok(i) => match Days::try_from_primitive(i) {
            Ok(v) => v,
            Err(_) => panic!("Expected integer"),
        },
        Err(_) => panic!("Expected integer"),
    }
}

arg_enum! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Parts {
        One = 1,
        Two = 2,
    }
}

fn parts_parser(s: &str) -> Parts {
    match u8::from_str(s) {
        Ok(i) => match Parts::try_from_primitive(i) {
            Ok(v) => v,
            Err(_) => panic!("Expected integer"),
        },
        Err(_) => panic!("Expected integer"),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Advent of Code")]
struct Opt {
    #[structopt(short, long, parse(from_str = days_parser), help = r"Day to run")]
    day: Days,
    #[structopt(short,long,parse(from_str = parts_parser),help = r"Part to run")]
    part: Parts,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let answer = match opt.day {
        Days::One => day01::run(opt.part as usize).await,
        Days::Two => day02::run(opt.part as usize).await,
    };

    println!("Day {} Part {}: {}", opt.day, opt.part, answer)
}
