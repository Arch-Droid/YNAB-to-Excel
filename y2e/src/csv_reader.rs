pub fn read_css() -> Result<String, String> {
    use std::fs::File;
    use csv::ReaderBuilder;

    let path = "/Users/francisco/Documents/Estudos/Programming/Rust/YNAB to Excel/transactions.csv";
    
    // Open the CSV file
    let file = File::open(path).expect("failed to find transactions file");

    // Create a reader
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    // Define a iterator
    let mut iter = reader.records();

    loop {
          // Iterate the reader
        match iter.next() {
            Some(result) => match result {
                Ok(record) => {
                    println!("{:?}",record);

                },
                Err(_) => return Err(String::from("Failure to propagate result"))
            },
            None => return Err(String::from("error grabbing result")),
            
        }
        
    }

}


//Works!
fn rob_sheet<'a> (worksheet: &'a xlsxwriter::Worksheet) -> &'a xlsxwriter::Worksheet<'a> {
    return worksheet;

}