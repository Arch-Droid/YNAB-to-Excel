mod csv_extraction;
mod excel;
use csv_extraction::read_css;
use excel::generate_excel;
// use excel::generate_excel;

fn main() {

   let path = "/Users/francisco/Documents/Scientia/Programming/Rust/YNAB to Excel/transactions.csv";

generate_excel(&path)

}
