
use std::io::{self};


pub fn read_css() -> Result<String, io::Error> {
    use std::fs::File;
    use csv::ReaderBuilder;

    let path = "/Users/francisco/Documents/Estudos/Programming/Rust/YNAB to Excel/transactions.csv";
    
    // Open the CSV file
    let file = File::open(path).expect("failed to find transactions file");

    // Create a reader
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    // Define a reader
    let mut iter = reader.records();

    // Iterate the reader
    if let Some(result) = iter.next(){
        let text = result.unwrap();
        print!("{:?}", text);
        Ok(String::new())
    } else {

        panic!("Failed to iterate next")

    }
}