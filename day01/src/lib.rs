use futures::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};
use utils::DayInput;

pub async fn run(part: usize) -> usize {
    let (d, p) = ("1", &part.to_string());
    let input = DayInput::new(d, p).await;

    match part {
        1 => part_1(input).await,
        2 => part_2(input).await,
        _ => 0,
    }
}

async fn part_1(input: DayInput) -> usize {
    let linestream = FramedRead::new(input.reader, LinesCodec::new());

    let tmp = linestream
        .map(|line| line.unwrap())
        .fold((0, String::new()), |acc, x| async move {
            if acc.1 < x {
                (acc.0 + 1, x)
            } else {
                (acc.0, x)
            }
        })
        .await;

    tmp.0
}

async fn part_2(input: DayInput) -> usize {
    let linestream = FramedRead::new(input.reader, LinesCodec::new());

    let tmp = linestream
        .map(|line| line.unwrap())
        .fold(
            (0, String::new(), String::new(), String::new()),
            |acc, x| async move {
                if acc.1 < x {
                    (acc.0 + 1, acc.2, acc.3, x)
                } else {
                    (acc.0, acc.2, acc.3, x)
                }
            },
        )
        .await;

    tmp.0
}
