mod csv_extraction;
mod excel;
use csv_extraction::read_css;
// use excel::generate_excel;

fn main() {
   match read_css(){
      Ok(text) => print!("{}", text),
      Err(error) => println!("{:?}", error),
   }

}
