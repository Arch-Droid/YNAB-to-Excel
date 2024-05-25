/*
Ouput:
Generates an Excel file necessary for accounting of the individual with all the necessary columns:
Date     Initials        Memo        Income      Expense     2-[Initials]        Category_number     Description_of_Category     FR.[Category Number]

Automatically crams category 14
Prompts wheter user wants to cram 15 (y/n)

Adds and individual row for each transaction

Input:
Takes an css file formated as YNAB exports it

Extraction:
Parses a CSS 'mutations.css' file by \t to a 'row': Vec<str>

Date is index 0 [-11:-1]                            of this row
Memo is index 9                                     of this row
Mutations (both income and expense )is index 10     of this row
Category_Number is index 7[:2]                      of this row

Description of category will be grabed from another css file 'description.css'

Generating output:
Uses the Workbook object directly as data is extracted so the extracting is writing and writing is extracting
This way there is no necessity of creating own objects and later assign them to the workbook object through the write_<type>() method


pub fn generate_excel(trasactions_css_paht_file: &str, description_css_paht_file: &str){
    
    //Include the main excel library
    use xlsxwriter::prelude::*;

    //Create the workbook which will become the excel file
    let workbook = Workbook::new("simple1.xlsx").unwrap();

    //Create a mutable sheet for the workbook.
    //All the writing will take place here
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

*/