use futures::StreamExt;
use std::str::FromStr;
use tokio_util::codec::{FramedRead, LinesCodec};
use utils::DayInput;

pub async fn run(part: usize) -> usize {
    let (d, p) = ("3", &part.to_string());
    let input = DayInput::new(d, p).await;

    match part {
        1 => part_1(input).await,
        _ => 0,
    }
}

async fn part_1(input: DayInput) -> usize {
    let linestream = FramedRead::new(input.reader, LinesCodec::new());

    let tmp = linestream
        .map(|line| {
            let l = line.unwrap();
            usize::from_str_radix(l, 2).unwrap()
        })
        .fold(
            (0, 0),
            |(total, count), val| async move { (total + val, count) },
        )
        .await;

    0
}
