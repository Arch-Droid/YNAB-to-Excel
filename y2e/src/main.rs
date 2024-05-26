mod csv_reader;
mod excel;
use csv_reader::read_css;
// use excel::generate_excel;

fn main() {
   match read_css(){
      Ok(text) => print!("{}", text),
      Err(error) => println!("{:?}", error),
   }

}
