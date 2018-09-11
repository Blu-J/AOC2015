use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
use errors::*;

enum CheckResult {
    Valid(HashMap<String, bool>),
    Invalid,
}

fn check(val: &str) -> bool {
    match val.split_whitespace()
        .fold(CheckResult::Valid(HashMap::new()), |result, val| {
            let mut val = val.as_bytes().to_vec();
            val.sort();
            let val = format!("{}", str::from_utf8(&val).unwrap());
            match result {
                CheckResult::Valid(mut hm) => {
                    if hm.contains_key(&val) {
                        CheckResult::Invalid
                    } else {
                        hm.insert(val, true);
                        CheckResult::Valid(hm)
                    }
                }
                _ => result,
            }
        }) {
        CheckResult::Valid(_) => true,
        _ => false,
    }
}

fn check_file(path: &str) -> Result<u32> {
    let input = File::open(path).chain_err(|| format!("Unable to open file {}", path))?;
    let buffered = BufReader::new(input);
    let count = buffered.lines().fold(0_u32, |ans, line| {
        if let Ok(value) = line {
            match check(value.as_str()) {
                true => ans + 1,
                false => ans,
            }
        } else {
            ans
        }
    });
    Ok(count)
}
fn main() {
    println!("There are {} valid lines", check_file("test.txt").unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(check("abcde fghij"), true);
    }
    #[test]
    fn a1() {
        assert_eq!(check("abcde xyz ecdab"), false);
    }
    #[test]
    fn a2() {
        assert_eq!(check("a ab abc abd abf abj"), true);
    }
    #[test]
    fn a3() {
        assert_eq!(check("iiii oiii ooii oooi oooo"), true);
    }
    #[test]
    fn a4() {
        assert_eq!(check("oiii ioii iioi iiio"), false);
    }
}
