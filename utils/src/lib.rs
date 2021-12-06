use std::path::Path;
use tokio::fs::File;
use tokio::io::BufReader;

pub struct DayInput {
    pub reader: BufReader<File>,
}

impl DayInput {
    pub async fn new(day: &str, part: &str) -> Self {
        let filepath = format!("./input/day{}_part{}.txt", day, part);
        let path = Path::new(&filepath);
        let file = File::open(path).await.unwrap();

        DayInput {
            reader: BufReader::new(file),
        }
    }
}
