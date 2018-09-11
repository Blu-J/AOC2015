use std::fmt;
#[derive(Debug, Copy, Clone)]
struct Pos(usize, usize);

struct Ans {
    values: Vec<Vec<Option<u64>>>,
    current: Pos,
    size: usize,
}

#[derive(Debug)]
enum ValueGet {
    OutOfBounds,
    None,
    Some(u64),
}
impl ValueGet {
    fn value_or(&self, or: u64) -> u64 {
        match self {
            &ValueGet::Some(x) => x,
            _ => or,
        }
    }
}

impl fmt::Debug for Ans {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self.values
                .iter()
                .map(|v| v.iter().map(|&x| x.unwrap_or(0)).collect::<Vec<u64>>())
                .collect::<Vec<Vec<u64>>>()
        )
    }
}

fn safe_sum(value: usize, with_value: i32) -> Option<usize> {
    if with_value < 0 {
        value.checked_sub(with_value.abs() as usize)
    } else {
        value.checked_add(with_value as usize)
    }
}

impl Ans {
    fn new() -> Ans {
        Ans {
            values: vec![vec![Some(1_u64)]],
            current: Pos(0, 0),
            size: 1,
        }
    }
    fn get(&self, offset: (i32, i32)) -> ValueGet {
        let position = (
            safe_sum(self.current.0, offset.0),
            safe_sum(self.current.1, offset.1),
        );
        match position {
            (Some(x), Some(y)) => {
                if x >= self.size {
                    ValueGet::OutOfBounds
                } else if y >= self.size {
                    ValueGet::OutOfBounds
                } else {
                    match self.values[y][x] {
                        Some(value) => ValueGet::Some(value),

                        None => ValueGet::None,
                    }
                }
            }
            _ => ValueGet::OutOfBounds,
        }
    }
    fn set(&mut self, value: u64) -> () {
        if self.current.0 >= self.size || self.current.1 >= self.size {
            panic!("Could not set with  self.current {:?}", self.current);
        } else {
            self.values[self.current.1][self.current.0] = Some(value);
        }
    }
    fn current_value(&self) -> u64 {
        self.values[self.current.1][self.current.0].unwrap()
    }
    fn next(&mut self) -> u64 {
        let needs_bigger_size = self.size - 1 == self.current.0 && self.size - 1 == self.current.1;
        if needs_bigger_size {
            self.size += 2;
            self.current = Pos(self.current.0 + 1, self.current.1 + 1);
            self.values.iter_mut().for_each(|row| {
                row.push(None);
                row.insert(0, None);
            });
            self.values.push(vec![None; self.size]);
            self.values.insert(0, vec![None; self.size]);
        }
        let next = {
            let top = self.get((0, -1));
            let bottom = self.get((0, 1));
            let right = self.get((1, 0));
            let left = self.get((-1, 0));
            match (&top, &right, &bottom, &left) {
                (_, &ValueGet::None, _, _) => {
                    self.current = Pos(self.current.0 + 1, self.current.1);
                }
                (&ValueGet::None, _, _, _) => {
                    self.current = Pos(self.current.0, self.current.1 - 1);
                }
                (_, _, _, &ValueGet::None) => {
                    self.current = Pos(self.current.0 - 1, self.current.1);
                }
                (_, _, &ValueGet::None, _) => {
                    self.current = Pos(self.current.0, self.current.1 + 1);
                }
                _ => {
                    panic!(
                        "There is a major problem we missed a state {:?} with ans= {:?}",
                        (&top, &right, &bottom, &left),
                        self
                    );
                }
            };
            (-1..2).fold(0, |acc, x| {
                acc + (-1..2).fold(0, |acc, y| acc + self.get((x, y)).value_or(0))
            })
        };
        self.set(next);
        return next;
    }
}

fn get_answer(val: u64) -> u64 {
    let mut ans = Ans::new();
    while ans.next() <= val {}
    ans.current_value()
}
fn main() {
    println!("We have the answer of {}", get_answer(325489))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a1() {
        assert_eq!(get_answer(1), 2);
    }
    #[test]
    fn a2() {
        assert_eq!(get_answer(2), 4);
    }
    #[test]
    fn a4() {
        assert_eq!(get_answer(4), 5);
    }
    #[test]
    fn a5() {
        assert_eq!(get_answer(5), 10);
    }
    #[test]
    fn a747() {
        assert_eq!(get_answer(747), 806);
    }
}
