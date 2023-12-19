use std::{
    fmt::Debug,
    io::{self, Read, Write},
};

pub fn help_with<I, O, Err, F>(f: F) -> impl FnMut(I) -> Result<O, Err>
where
    F: Fn(I) -> Result<O, Err>,
    I: Debug + Clone,
    O: serde::de::DeserializeOwned,
    Err: Debug,
{
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();
    _help_with(f, input, output)
}

/// for testing, we want to be able to pass in different read and write
fn _help_with<I, O, Err, F>(
    f: F,
    mut read: impl Read,
    mut write: impl Write,
) -> impl FnMut(I) -> Result<O, Err>
where
    F: Fn(I) -> Result<O, Err>,
    I: Debug + Clone,
    O: serde::de::DeserializeOwned,
    Err: Debug,
{
    move |i: I| match f(i.clone()) {
        Ok(o) => Ok(o),
        Err(err) => {
            let mut err = format!("{err:?}");
            let mut i = format!("{i:?}");

            let o: O = loop {
                writeln!(write, "Error with input {i}: {err}").expect("Failed to write");
                write!(write, "Fix: ").expect("Failed to write");
                write.flush().expect("Failed to flush");

                let mut string = String::new();
                read.read_to_string(&mut string).expect("Failed to read");

                match serde_json::from_str(&string) {
                    Ok(o) => break o,
                    Err(e) => {
                        err = format!("{e:?}");
                        i = string;
                    }
                }
            };

            Ok(o)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::*;

    fn parse_num(s: &str) -> Result<u8, ParseIntError> {
        s.parse()
    }

    #[test]
    fn non_erroring_works() {
        let result = help_with(parse_num)("1");
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn erroring_works() {
        let input = "3".as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let result = _help_with(parse_num, input, &mut output)("three");

        assert_eq!(result, Ok(3));
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "Error with input \"three\": ParseIntError { kind: InvalidDigit }\nFix: "
        )
    }

    #[test]
    fn complex_type() {
        #[derive(serde::Deserialize, Debug, PartialEq, Eq)]
        struct MyType {
            string: String,
            num: u8,
        }

        let input = "{\"string\": \"my string\", \"num\": 34}".as_bytes();
        let mut output: Vec<u8> = Vec::new();

        // we dont really have a function that parses this, so we just pass a closure that errors
        let result =
            _help_with(|_t| Err("we dont have a parser"), input, &mut output)("[my string, 34]");

        assert_eq!(
            result,
            Ok(MyType {
                string: "my string".to_string(),
                num: 34
            })
        );
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "Error with input \"[my string, 34]\": \"we dont have a parser\"\nFix: "
        )
    }
}
