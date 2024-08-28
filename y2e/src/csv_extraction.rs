/* 

pub fn write_to_sheet:
takes the &iterator.next()
takes the sheet on which to write on
takes the row_index to write to the right place

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

Data is validated as extrated, errors are dealt at extraction level
Writers call panic!() if data is invalid

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


use std::{io, u32};
use csv::DeserializeRecordsIter;


//Writes every row
// Sheet is the mutable sheet
// Row is the raw data which needs to be processed
// Index is the height of the row NOT column, columns are specifed in each individual write function
pub fn write_to_sheet<'a>(sheet: &mut xlsxwriter::Worksheet, 
                        row: &Vec<String>,
                        index: u32, cram_14: &mut Vec<f32>, cram_15: &mut Vec<f32>) -> Result<(),String>{

    //Preliminary check (only ABN allowed)
    check_account(row)?;
    check_date(row, "01/07/2024")?;     // Date to compare

    // Check wheather category equals 14 to cram
    if extract_category(row) == Ok("14".to_string()){
        println!("14");
        cram_income(cram_14, row, sheet, 0);
        cram_expense(cram_14, row, sheet, 0);
        
        return Err("CRAM".to_string());
        
    // Check wheather category equals 15 to cram
    } else if extract_category(row) == Ok("15".to_string()) && promt_user(row){
        println!("15");
        cram_income(cram_15, row, sheet, 1);
        cram_expense(cram_15, row, sheet, 1);

        return Err("CRAM".to_string());

    } else {
    write_date(row, sheet, index)?;
    write_initals(row, sheet, index);
    write_memo(row, sheet, index);
    write_income(row, sheet, index);
    write_expense(row, sheet, index);
    write_2_initals(row, sheet, index);
    write_category_number_and_initials_dot_category_number(row, sheet, index);
    }
    
    Ok(())
}


//
// Functions responsible for writing on the sheet
//

fn write_date<>(row: &Vec<String>, sheet: &mut xlsxwriter::Worksheet, index: u32) -> Result<(),String>{

    // Define a date
    let date: &String;

    // Extracts date and propagates error
    date = extract_date(row)?;
    
    if compare_date(&date, "20/05/2024".to_string())?{
        // Write date
        sheet.write_string(index, 0, &date, None).expect("Failed to write");
        Ok(())

    } else{
        Err("Date older".to_string())
    }

}

fn write_initals(row: &Vec<String>, sheet: &mut xlsxwriter::Worksheet, index: u32){
    // Write initials
    // Hardcoded
    sheet.write_string(index, 1, "FR", None).unwrap()
    
}

fn write_memo(row: &Vec<String>, sheet: &mut xlsxwriter::Worksheet, index: u32){
    // Extract memo
    // Does not deal with error
    // Write Memo

    let memo: String;

    match extract_memo(row){
        Ok(value) => memo = value,
        Err(text) => panic!("{text}"),
    }

    sheet.write_string(index, 2, &memo, None).unwrap()
}

fn write_income(row: &Vec<String>, sheet: &mut xlsxwriter::Worksheet, index: u32){
    // Extracts income
    // Writes income
    let income: &String;

    match extract_income(row){
        Ok(value) => income = value,
        Err(text) => panic!("{text}"),
    }

    sheet.write_string(index, 3, &income, None).unwrap()
}

fn write_expense(row: &Vec<String>, sheet: &mut xlsxwriter::Worksheet, index: u32){
    // Extracts expense
    // Writes expense
    let expense: &String;

    match extract_expense(row){
        Ok(value) => expense = value,
        Err(text) => panic!("{text}"),
    }

    sheet.write_string(index, 4, expense, None).unwrap()
}

fn write_2_initals(row: &Vec<String>, sheet: &mut xlsxwriter::Worksheet, index: u32){
    // Write '2-[initials]
    // Hardcoded
    sheet.write_string(index, 5, "2-FR", None).unwrap()
    
}

fn write_category_number_and_initials_dot_category_number(row: &Vec<String>, sheet: &mut xlsxwriter::Worksheet, index: u32){
    // Extracts category
    // Does not deal with error
    // Writes category number
    
    let category: String;
    
    match extract_category(row){
        Ok(value) => category = value,
        Err(text) => panic!("{text}"),
    }

    sheet.write_string(index, 6, &category, None).unwrap();
    write_initials_dot_category_number(category, sheet, index);
    
}

fn write_description_of_category(row: &Vec<String>, sheet: &mut xlsxwriter::Worksheet, index: u32){
    // Extracts discription from second csv file:
    // 'descriptions.csv'
    // Writes description of category

    //Not implemented

    extract_category_description();
}

fn write_initials_dot_category_number(category_number: String, sheet: &mut xlsxwriter::Worksheet, index: u32, ){
    // Takes extracted category number
    // Writes '[Initials].category_number'

    let initials_dot_category_number = "FR.".to_string() + &category_number;

    sheet.write_string(index, 7, &initials_dot_category_number, None).unwrap()

}

// Writer Functions responsible for cramming
//
//

// Crams income accesing stored higer cram values
// Takes the original value (cram), the row information, the sheet to write on, and the index of the row on which to write (in this case only 1 or 2)

fn cram_income(cram: &mut Vec<f32>, row: &Vec<String>, sheet:&mut xlsxwriter::Worksheet, index: u32){

    // Define left half of income
    let income  = &extract_income(row).unwrap()
                                                .chars()
                                                .filter(|&c| c != '.' && c != ' ')
                                                .map(|c| if c == ',' { '.' } else if c == '€' { ' ' } else {c})
                                                .collect::<String>();
    println!("Income: {}", income);
    let income_as_f32: f32 = income.trim().parse().expect("TROUBLE");

    // Add the values up
    cram[0] += income_as_f32;


    //Write the total income
    sheet.write_string(index, 3, &cram[0].to_string(), None).unwrap()
}

// Crams expense accesing cram values stored higher up
fn cram_expense(cram: &mut Vec<f32>, row: &Vec<String>, sheet:&mut xlsxwriter::Worksheet, index: u32){

    // Expense, rewrites the comma to a dot
    let expense  = &extract_expense(row).unwrap()
                                                .chars()
                                                .filter(|&c| c != '.' && c != ' ')
                                                .map(|c| if c == ',' { '.' } else if c == '€' { ' ' } else {c})
                                                .collect::<String>();
    println!("Expense: {}", expense);
    let expense_as_f32: f32 = expense.trim().parse().expect("TROUBLE");

    // Add the values up
    println!("Cram before: {}", cram[1]);
    cram[1] = cram[1] + expense_as_f32;
    println!("Cram: {}", cram[1]);

    sheet.write_string(index, 4, &cram[1].to_string(), None).unwrap()
}


//
// Functions responsible for extracting information from csv_file 
//
//


// Extracts account

fn extract_account(row: &Vec<String>) -> Result<&String, &str>{
    let index = 0; // Account

    match row.get(index) {                  
        Some(value) => Ok(value),
        None => Err("index out of range"),
    }
}


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

fn extract_memo(row: &Vec<String>) -> Result<String, &str>{
    let index = 8;      // Memo

    let mut memo = String::new();

    if let Some(value) = row.get(index) {
        memo = String::from(value);
        Ok(memo)
        
    } else{
        print!("Memo not found, adding PLACEHOLDER ");
        Ok("MEMO".to_string())

    }

    //TODO: deal with empty Memo
        

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
// Instead of returning the string reference
// returns a new string value[0..2].to_string

fn extract_category(row: &Vec<String>) -> Result<String, &str>{

    let index = 7;   // category

    let mut category = String::new();

    match row.get(index) {                  
        Some(value) => {

            if value.len() > 2 {    // Checks whether the lenght of sub_category is > 2
                category = value[0..2].to_string();
                Ok(category)

            } else {

                let valid = false;
                
                // Warn the user
                println!("Category not found, adding PLACEHOLDER...");
                Ok("CATEGORY".to_string())

                }
                    
            }
        
        None => Err("index out of range"),
        
    }
        
}

fn extract_category_description(){
    // Not implemented
    // Wil be implemented later

}



// Functions responsible for logic
//
//

// Checks whether account is ABN

fn check_account(row: &Vec<String>) -> Result<(),String>{

    let index = 0; // Account

    match row.get(index){
        Some(account) => {
            if account == "ABN"{
                Ok(())
            } else {
                Err("Wrong account".to_string())
            }
        }
        None => Err("Index out of range".to_string()),
    }
}

// Checks wheather date is outdated

fn check_date(row: &Vec<String>, comparision: &str) -> Result<(), String>{

    // Extracts date and propagates error
    let date = extract_date(row)?;

    if compare_date(&date, comparision.to_string())?{
        Ok(())

    } else{
        Err("Date older".to_string())
    }

}

// Compares a date to a comparision
fn compare_date(date: &String, comparision: String) -> Result<bool, String>{       // TODO: Why String works and &str gives so much trouble
    
    // Coversion Dates to vectors

    let date_as_vector: Vec<u32> = format_date_to_vector(&date)?;
    let comparision_as_vector: Vec<u32> = format_date_to_vector(&comparision)?;

    //Compares the dates
    // If year is smaller then the date is older than the fixed comparision
    if date_as_vector[2] < comparision_as_vector[2]{
        Ok(false)
    // If year is the same but month is smaller then the date is older than the fixed comparision
    } else if date_as_vector[2] == comparision_as_vector[2] && date_as_vector[1] < comparision_as_vector[1] {
        Ok(false)
    // If the year is not smaller nor equal than the date is younger
    } else {
        Ok(true)
    }
}


//Prompts user about cramming 15

fn promt_user(row: &Vec<String>) -> bool{
    println!("The following category has been identified as 15, do you wish to cram it?");
    println!("{:?}", row);

    let mut awnser = String::new();
    io::stdin()
        .read_line(&mut awnser)
        .expect("Failed to read line");

    if awnser == "y\n".to_string(){
        return true
    }

    else {
        return false
    }

}

//Functions responsible for formatting data

fn format_date_to_vector(date: &String) -> Result<Vec<u32>, &str>{

    // Alocate a Vector
    let mut date_vector: Vec<u32> = vec![0,0,0];

    // Separates date into Vec<u32>[xx, xx, xxxx]

    if date.len() > 1{                  // Checks wheter first element is present: xx-00-0000
        if let Ok(value) = date[0..2].parse(){
            date_vector[0] = value;
        } else{
            return Err("Failed to parse first 2 digits of date");
        }
    } else{
        return Err("Lenght of date is too short, first element not found");
    }
    
    if date.len() > 4{                  // Checks wheter second element is present: 00-xx-0000
        if let Ok(value) = date[3..5].parse(){
            date_vector[1] = value;
        } else{
            return Err("Failed to parse second 2 digits of date");
        }
    } else{
        return Err("Lenght of date is too short, second element not found");
    }
    
    if date.len() > 9{                  // Checks wheter third element is present: 00-00-xxxx
        if let Ok(value) = date[6..10].parse(){
            date_vector[2] = value;
        } else{
            return Err("Failed to parse last 4 digits of date");
        }
    } else{
        return Err("Lenght of date is too short, third element not found");
    }

    return Ok(date_vector);
}

// Deprecated will be removed in the future
pub fn read_css() -> Result<String, String> {
    use std::fs::File;
    use csv::ReaderBuilder;

    let path = "/Users/francisco/Documents/Scientia/Programming/Rust/YNAB to Excel/transactions.csv";
    
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

                    print!("{:?}",&record[3]);  // Date
                    print!("{:?}",extract_category(&record));    // Category
                    print!("{:?}",record[8]);           // Memo
                    print!("{:?}",record[9]);           // Expense
                    println!("{:?}",record[10]);        // Income
                    
                },
                Err(_) => return Err(String::from("Failure to retrieve result"))
            },
            None => return Err(String::from("error iterating")),
            
        }
        
    }

}