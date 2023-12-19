use helping_hand::help_with;
use std::num::ParseIntError;

fn parse_num(s: &str) -> Result<u8, ParseIntError> {
    s.parse()
}

fn main() {
    for num in ["1", "2", "three", "4"] {
        let result = help_with(parse_num)(num).unwrap();
        println!("{result}");
    }
}
