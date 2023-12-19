use helping_hand::help_with;
use serde::Deserialize;

#[derive(Deserialize)]
struct Repeat {
    string: String,
    times: usize,
}

fn parse(s: &str) -> Result<Repeat, &'static str> {
    let vec = s.split(',').collect::<Vec<_>>();

    let [string, times] = vec[..] else {
        return Err("incorrect number of commas");
    };

    let string = string.to_string();
    let times = times.parse().map_err(|_| "error parsing number")?;

    Ok(Repeat { string, times })
}

fn main() {
    for num in ["hello,1", "world,2", "this has a semicolon instead!; 4"] {
        let result = help_with(parse)(num).unwrap();
        println!("{}", result.string.repeat(result.times));
    }
}
