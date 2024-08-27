/*
Ouput:
Generates an Excel file necessary for accounting of the individual with all the necessary columns:
Date     Initials        Memo        Income      Expense     2-[Initials]        Category_number     Description_of_Category     FR.[Category Number]

Automatically crams category 14
Prompts wheter user wants to cram 15 (y/n)

Adds and individual row for each transaction

Input:
Takes a csv file formated as YNAB exports it

Extraction:
Parses a csv 'mutations.csv' file by \t and serializes to a 'row': Vec<String>

Date is index 3                                     of this row
Memo is index 8                                     of this row
Expense is index 9                                  of this row
Income is index 10                                  of this row
Category_Number is index 7[:2]                      of this row

Description of category will be grabed from another css file 'description.csv'

Generating output:
Runs a loop
rows{

    csv_extraction::write_to_sheet(sheet: &mut Workbook::Sheet, row: Vec<String>, index: u32)
        // data is extracted and written to a new row on the sheet:
        // 14 and 15 managed appropriately in sub-functions
    
    row = iterator.next()
}
calls workbook.close() and finishes


Extraction:
For fruther documentation see csv_extraction.

CSV file and iterator are opened in pub fn generate_excel
passed as (mutable) reference to the sub protocols

*/

  use csv::DeserializeRecordsIter;

//Include csv extraction
  use crate::csv_extraction::write_to_sheet;

pub fn generate_excel(trasactions_css_paht_file: &str){
    
    //Include the main excel library
    use xlsxwriter::prelude::*;

    //Include the standard File
    use std::fs::File;

    //Include the csv library
    use csv::ReaderBuilder;



    // Open the CSV file
    let file = File::open(trasactions_css_paht_file).expect("failed to find transactions file");

    // Create a reader
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(file);
    
    // Define a iterator
    let mut iter:DeserializeRecordsIter<File, Vec<String>> = reader.deserialize();

    
    //Create the workbook which will become the excel file
    let workbook = Workbook::new("simple1.xlsx").unwrap();

    //Create a mutable sheet for the workbook.
    let mut sheet = workbook.add_worksheet(None).unwrap();

    
    //We introduce an index to write to the appropriate row
    // index = 0 reserved for 14
    // index = 1 reserved for 15
    let mut index = 3;

    // Stored values for cramming 14 and 15
     let mut cram_14: Vec<f32> = vec![0.0,0.0];
     let mut cram_15: Vec<f32> = vec![0.0,0.0];

    //Iterate the iterator writing everything
    for element in iter{
        match element{
            Ok(row) => {
                match write_to_sheet(&mut sheet, &row, index, &mut cram_14, &mut cram_15){
                    Ok(_) => index += 1,
                    Err(text) => {

                    },
                }
            }
            Err(_) => (),
        }
    }

    workbook.close().unwrap();

}