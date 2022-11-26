use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILE_NAME: &str = "history.csv";

#[derive(Debug)]
struct HistoryData {
    data_type: String,
    tag: String,
    text: String,
    life: i32,

}

fn main() {
    let mut history_data: Vec<HistoryData> = Vec::new();
    
    let content = fs::read_to_string(FILE_NAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let data = HistoryData{
            data_type: result.as_ref().unwrap().get(0).unwrap().to_string(),
            tag: result.as_ref().unwrap().get(1).unwrap().to_string(),
            text: result.as_ref().unwrap().get(2).unwrap().to_string(),
            life: 0
        };

        history_data.push(data);
    }

    println!("{:?}", history_data);
    
}
