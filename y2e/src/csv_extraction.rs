/* 

pub fn write_to_sheet:
takes the &iterator.next()
takes the sheet on which to write on
takes the row_index

extracts from &iterator.next:
    Account
        if != "ABN" return Err
    Date
        if < set date return Err
    Sub Category [:2]
        if 14 -> deal differently on writing
        if 15 -> prompt wheather wants to cram
            if yes -> deal as 14\
    Memo
    Outlfow
    Inflow

writes on row_index
    colomn 0 -> write_Date()
    colomn 1 -> write_initals()
    colomn 2 -> write_Memo()
    colomn 3 -> write_Income()
    colomn 4 -> write_expense()
    colomn 5 -> write_2-[initials]()
    colomn 6 -> write_category_number()
    colomn 7 -> write_description of category()
    colomn 8 -> write_Fr.[category_number]()


*/

use csv::DeserializeRecordsIter;
use xlsxwriter::Format;


fn write_to_sheet<'a>(sheet: &mut xlsxwriter::Worksheet, 
                        row: &Vec<String>) {
}


//
// Functions responsible for writing on the sheet
//

fn write_date(){
    // Extract date 
    // Write date
}

fn write_initals(){
    // Write initials
    // Hardcoded
}

fn write_memo(){
    // Extract memo
    // Deal with error
    // Write Memo
}

fn write_income(){
    // Extracts income
    // Writes income
}

fn write_expense(){
    // Extracts expense
    // Writes expense
}

fn write_2_initals(){
    // Write '2-[initials]
    // Hardcoded
}

fn write_category_number(){
    // Extracts category
    // Deals with error
    // Writes category number

}

fn write_description_of_category(){
    // Extracts discription from second csv file:
    // 'descriptions.csv'
    // Writes description of category
}

fn write_initials_dot_category_number(){
    // Takes extracted category number
    // Writes '[Initials].category_number'
}


//
// Functions responsible for extracting information from csv_file 
//

//Extracts Date to a String xx-xx-xxxx

fn extract_date(row: &Vec<String>) -> Result<&String, &str>{
    let index = 3;      // Date


    match row.get(index) {                  
        Some(value) => Ok(value),
        None => Err("index out of range"),
    }
        

}

// Not implemented: fn extract_initals()


//Extracts memo to a string
fn extract_memo(row: &Vec<String>) -> Result<&String, &str>{
    let index = 8;      // Memo


    match row.get(index) {                  
        Some(value) => Ok(value),
        None => Err("index out of range"),
    }
        

}

//Extracts income to a string
fn extract_income(row: &Vec<String>) -> Result<&String, &str>{
    let index = 10;     // Income

    match row.get(index) {                  
        Some(value) => Ok(value),
        None => Err("index out of range"),
    }
        

}


//Extracts expense to a string
fn extract_expense(row: &Vec<String>) -> Result<&String, &str>{
    let index = 9;      // Expense
    match row.get(index) {                  
        Some(value) => Ok(value),
        None => Err("index out of range"),
    }
        

}

// Extracts category, with extra checks
//
// Instead of returning the string reference pointing to the equivalent element of the iterator,
// returns a new string value[0..2].to_string
fn extract_category(row: &Vec<String>) -> Result<String, &str>{

    let index = 7;   // category

    match row.get(index) {                  
        Some(value) => {

            if value.len() > 2 {                // Checks whether the lenght of sub_category is > 2
                Ok(value[0..2].to_string())

            } else {
                Err("No category found")
            }
        }

        None => Err("index out of range"),
    }
        

}

fn extract_category_description(){

    //Wil be implemented later

}

// Deprecated will be removed in the future
pub fn read_css() -> Result<String, String> {
    use std::fs::File;
    use csv::ReaderBuilder;

    let path = "/Users/francisco/Documents/Estudos/Programming/Rust/YNAB to Excel/transactions.csv";
    
    // Open the CSV file
    let file = File::open(path).expect("failed to find transactions file");

    // Create a reader
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    // Define a iterator
    let mut iter:DeserializeRecordsIter<File, Vec<String>> = reader.deserialize();

    loop {
          // Iterate the reader
        match iter.next() {
            Some(result) => match result {
                Ok(record) => {
                    print!("{:?}",record[3]);           // Date
                    print!("{:?}",&record[7][0..2]); // Category
                    print!("{:?}",record[8]);           // Memo
                    print!("{:?}",record[9]);           // Expense
                    println!("{:?}",record[10]);          // Income
                    
                },
                Err(_) => return Err(String::from("Failure to retrieve result"))
            },
            None => return Err(String::from("error iterating")),
            
        }
        
    }

}


//Works!
fn rob_sheet<'a> (worksheet: &'a xlsxwriter::Worksheet) -> &'a xlsxwriter::Worksheet<'a> {
    return worksheet;

}