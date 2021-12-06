use futures::StreamExt;
use std::str::FromStr;
use tokio_util::codec::{FramedRead, LinesCodec};
use utils::DayInput;

pub async fn run(part: usize) -> usize {
    let (d, p) = ("2", &part.to_string());
    let input = DayInput::new(d, p).await;

    match part {
        1 => part_1(input).await,
        2 => part_2(input).await,
        _ => 0,
    }
}

#[derive(Default, Clone, Copy)]
struct Point {
    x: isize,
    depth: isize,
    aim: isize,
}

struct Command {
    message: Message,
    units: isize,
}

#[derive(Debug, PartialEq)]
enum Message {
    Forward,
    Up,
    Down,
}

impl FromStr for Message {
    type Err = ();

    fn from_str(input: &str) -> Result<Message, Self::Err> {
        match input {
            "forward" => Ok(Message::Forward),
            "up" => Ok(Message::Up),
            "down" => Ok(Message::Down),
            _ => Err(()),
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let (a, b) = input.split_once(" ").unwrap();
        Ok(Command {
            message: Message::from_str(a).unwrap(),
            units: b.parse::<isize>().unwrap(),
        })
    }
}

impl Point {
    fn process_no_aim(&self, command: Command) -> Self {
        match command.message {
            Message::Forward => Self {
                x: self.x + command.units,
                depth: self.depth,
                aim: 0,
            },
            Message::Up => Self {
                x: self.x,
                depth: self.depth - command.units,
                aim: 0,
            },
            Message::Down => Self {
                x: self.x,
                depth: self.depth + command.units,
                aim: 0,
            },
        }
    }

    fn process_with_aim(&self, command: Command) -> Self {
        match command.message {
            Message::Forward => Self {
                x: self.x + command.units,
                depth: self.depth + (self.aim * command.units),
                aim: self.aim,
            },
            Message::Up => Self {
                x: self.x,
                depth: self.depth,
                aim: self.aim - command.units,
            },
            Message::Down => Self {
                x: self.x,
                depth: self.depth,
                aim: self.aim + command.units,
            },
        }
    }
}

async fn part_1(input: DayInput) -> usize {
    let linestream = FramedRead::new(input.reader, LinesCodec::new());

    let tmp = linestream
        .map(|line| {
            let l = line.unwrap();
            Command::from_str(&l).unwrap()
        })
        .fold(Point::default(), |point, command| async move {
            Point::process_no_aim(&point, command)
        })
        .await;

    (tmp.x * tmp.depth).try_into().unwrap()
}

async fn part_2(input: DayInput) -> usize {
    let linestream = FramedRead::new(input.reader, LinesCodec::new());

    let tmp = linestream
        .map(|line| {
            let l = line.unwrap();
            Command::from_str(&l).unwrap()
        })
        .fold(Point::default(), |point, command| async move {
            Point::process_with_aim(&point, command)
        })
        .await;

    (tmp.x * tmp.depth).try_into().unwrap()
}
