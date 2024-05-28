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

    csv_extraction::write_to_sheet(sheet: &mut Workbook::Sheet, row: Vec<String>)
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

pub fn generate_excel(trasactions_css_paht_file: &str, description_css_paht_file: &str){
    
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
    let mut iter = reader.records();

    //Calculate lenght of iterator



    //Create the workbook which will become the excel file
    let workbook = Workbook::new("simple1.xlsx").unwrap();

    //Create a mutable sheet for the workbook.
    let mut sheet1 = workbook.add_worksheet(None).unwrap();
    sheet1.write_string(0, 
                        0, 
                        "Red text", 
                        Some(&Format::new().set_font_color(FormatColor::Red))).unwrap();
    sheet1.write_number(0, 
                        1, 
                        20., 
                        None)
                        .unwrap();
    sheet1.write_formula_num(
                            1, 
                            0, 
                            "=10+B1", 
                            None, 
                            30.)
                            .unwrap();
    sheet1.write_url(
        1,
        1,
        "https://github.com/informationsea/xlsxwriter-rs",
        Some(&Format::new().set_font_color(FormatColor::Blue).set_underline(FormatUnderline::Single)),
    ).unwrap();
    sheet1.merge_range(2, 0, 3, 2, "Hello, world", Some(
        &Format::new().set_font_color(FormatColor::Green).set_align(FormatAlignment::CenterAcross)
                    .set_vertical_align(FormatVerticalAlignment::VerticalCenter))).unwrap();

    sheet1.set_selection(1, 0, 1, 2);
    sheet1.set_tab_color(FormatColor::Cyan);
    workbook.close().unwrap();

}

