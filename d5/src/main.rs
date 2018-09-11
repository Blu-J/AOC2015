use std::io::Read;
use std::fs::File;
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

#[derive(Debug)]
struct A {
    list: Vec<i32>,
    i: i32,
    count: u32,
}

impl A {
    fn new(list: Vec<i32>) -> A {
        A {
            list,
            i: 0,
            count: 0,
        }
    }
    fn in_list(&self) -> bool {
        !(self.i < 0 || self.i as usize >= self.list.len())
    }
    fn next(&mut self) {
        if !self.in_list() {
            return;
        }
        let next_i = self.list[self.i as usize] + self.i;
        self.count += 1;
        self.list[self.i as usize] += if self.list[self.i as usize] >= 3 {
            -1
        } else {
            1
        };
        self.i = next_i;
        // println!("Next shape is {:#?}", self);
    }
}
fn read_file(path: &str) -> Result<u32> {
    let mut input = File::open(path).chain_err(|| format!("Unable to open file {}", path))?;
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .chain_err(|| "Could not read the string into content")?;
    Ok(get_answer(content.as_str()))
}

fn get_answer(content: &str) -> u32 {
    let mut answer = A::new(
        content
            .lines()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>(),
    );
    while answer.in_list() {
        answer.next();
    }
    answer.count
}
fn main() {
    println!("Getting answer of {}", read_file("test.txt").unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn primary_example() {
        assert_eq!(
            get_answer(
                "0
3
0
1
-3"
            ),
            10
        );
    }
}
