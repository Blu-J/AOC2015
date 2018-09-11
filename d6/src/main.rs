use std::fmt::Debug;
use std::fmt;
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

#[derive(Debug)]
struct NeededInfo {
    biggest: i32,
    at_index: usize,
}

fn list_to_needed_info(list: &Vec<i32>) -> NeededInfo {
    list.iter().enumerate().fold(
        NeededInfo {
            biggest: i32::min_value(),
            at_index: 0,
        },
        |acc, (i, &val)| {
            if val > acc.biggest {
                NeededInfo {
                    biggest: val,
                    at_index: i,
                }
            } else {
                acc
            }
        },
    )
}

#[derive(Eq, PartialEq, Hash)]
struct Banks {
    list: Vec<i32>,
}
impl Debug for Banks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Banks ({})",
            self.list
                .iter()
                .skip(1)
                .fold(format!("{}", self.list[0]), |acc, val| format!(
                    "{} {}",
                    acc, val
                ))
        )
    }
}

impl Banks {
    fn new(list: Vec<i32>) -> Banks {
        Banks { list }
    }
    fn index_of(&self, set: &Vec<Banks>) -> Option<usize> {
        for (index, value) in set.into_iter().enumerate() {
            if value == self {
                return Some(index);
            }
        }
        None
    }
    fn next(&self) -> Banks {
        let needed_info = list_to_needed_info(&self.list);
        let list_len = self.list.len() as i32;
        let pieces = needed_info.biggest / list_len;
        let roll_around_extra = (needed_info.biggest % list_len) as usize;
        let upper_bound = (needed_info.at_index + roll_around_extra) as i32;
        let upper_bound_mod = upper_bound - list_len;
        Banks::new(
            self.list
                .iter()
                .enumerate()
                .map(|(x, &val)| {
                    let add_value = if (x > needed_info.at_index && (x as i32) <= upper_bound)
                        || ((x as i32) <= upper_bound_mod)
                    {
                        pieces + 1
                    } else {
                        pieces
                    };
                    if x == needed_info.at_index {
                        add_value
                    } else {
                        val + add_value
                    }
                })
                .collect::<Vec<i32>>(),
        )
    }
}

fn get_answer(content: &str) -> u32 {
    let mut banks = Banks::new(
        content
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>(),
    );
    let mut answer = 0;
    let mut founds = vec![];
    while banks.index_of(&founds).is_none() {
        answer += 1;
        let my_banks = banks;
        banks = my_banks.next();
        founds.push(my_banks);
    }
    return (answer - banks.index_of(&founds).unwrap()) as u32;
}
fn main() {
    println!(
        "Getting answer of {}",
        get_answer("2	8	8	5	4	2	3	1	5	5	1	2	15	13	5	14")
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn primary_example() {
        assert_eq!(get_answer("0 2 7 0"), 4);
    }

    #[test]
    fn step_1() {
        assert_eq!(
            Banks::new(vec![0, 2, 7, 0]).next(),
            Banks::new(vec![2, 4, 1, 2])
        );
    }
    #[test]
    fn step_2() {
        assert_eq!(
            Banks::new(vec![2, 4, 1, 2]).next(),
            Banks::new(vec![3, 1, 2, 3])
        );
    }
    #[test]
    fn step_3() {
        assert_eq!(
            Banks::new(vec![3, 1, 2, 3]).next(),
            Banks::new(vec![0, 2, 3, 4])
        );
    }
    #[test]
    fn step_4() {
        assert_eq!(
            Banks::new(vec![0, 2, 3, 4]).next(),
            Banks::new(vec![1, 3, 4, 1])
        );
    }
    #[test]
    fn step_5() {
        assert_eq!(
            Banks::new(vec![1, 3, 4, 1]).next(),
            Banks::new(vec![2, 4, 1, 2])
        );
    }
}
