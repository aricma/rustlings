// from_str.rs
// This is similar to from_into.rs, but this time we'll implement `FromStr`
// and return errors instead of falling back to a default value.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;
use std::str::FromStr;

trait Concat {
    fn concat(self: &Self, char: char) -> String;
}

impl Concat for String {
    fn concat(self: &Self, char: char) -> String {
        let mut new_string = self.clone();
        new_string.push(char);
        new_string
    }
}

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if you want to return a
// string error message, you can do so via just using return `Err("my error message".into())`.

enum ParsingState {
    Init,
    Name(String),
    NameWithPendingAge(String),
    NameAndAge(String, String),
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        s.to_string().chars().fold(Result::Ok(ParsingState::Init), |result, char| {
            match result {
                Ok(state) => match state {
                    ParsingState::Init => match char {
                        'a'..='z' | 'A'..='Z' => Result::Ok(ParsingState::Name(char.to_string())),
                        _ => Result::Err(ParsePersonError::NoName),
                    },
                    ParsingState::Name(name) => match char {
                        'a'..='z' | 'A'..='Z' => Result::Ok(ParsingState::Name(name.concat(char))),
                        _ => Result::Ok(ParsingState::NameWithPendingAge(name)),
                    },
                    ParsingState::NameWithPendingAge(name) => Result::Ok(ParsingState::NameAndAge(name, char.to_string())),
                    ParsingState::NameAndAge(name, age) => match char {
                        ',' => Result::Err(ParsePersonError::BadLen),
                        _ => Result::Ok(ParsingState::NameAndAge(name, age.concat(char))),
                    },
                },
                Err(err) => Result::Err(err)
            }
        }).and_then(|state| {
            match state {
                ParsingState::Init => Result::Err(ParsePersonError::Empty),
                ParsingState::NameAndAge(name, age) => {
                    match age.parse::<usize>() {
                        Ok(age) => Result::Ok(Person { name, age }),
                        Err(err) => Result::Err(ParsePersonError::ParseInt(err)),
                    }
                },
                ParsingState::NameWithPendingAge(name) => {
                    match "".to_string().parse::<usize>() {
                        Ok(age) => Result::Ok(Person { name, age }),
                        Err(err) => Result::Err(ParsePersonError::ParseInt(err)),
                    }
                },
                _ => Result::Err(ParsePersonError::BadLen)
            }
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
