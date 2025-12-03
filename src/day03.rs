use std::cmp::Ordering;

use crate::mass_parser::read_lines;

const NUMBERS_IN_U64: u32 = 19;

pub struct Rank {
    pub number_parts: Vec<u64>,
}

fn number_at(x: u64, pos: u32) -> u32 {
    let x_length = if x == 0 { 1 } else { x.ilog(10) + 1 };
    let mut trimmed_x = x / (10 as u64).pow(x_length - 1 - pos);
    trimmed_x = trimmed_x % 10;
    return trimmed_x as u32;
}

#[derive(Clone)]
#[derive(Debug)]
pub struct IndexedDigit {
    pub index: u32,
    pub inner_pos: u32,
    pub digit: u64,
}

impl IndexedDigit {
    pub fn compare_in_order(&self, other: &IndexedDigit) -> Ordering {
        let mut ord = self.digit.cmp(&other.digit);
        ord =
            if ord != Ordering::Equal { ord } else {
                other.index.cmp(&self.index)
            };
        ord = 
            if ord != Ordering::Equal { ord } else {
                other.inner_pos.cmp(&self.inner_pos)
            };
        ord
    }
}

impl Rank {

    fn largest_joltage(self) -> u32 {
        let largest_digit = self.number_parts
            .iter()
            .zip((0 as u32)..)
            .flat_map(|(part, vec_index)| (0..NUMBERS_IN_U64.min(part.ilog(10) + 1))
                .map(move |pos|
                    IndexedDigit { index: vec_index, inner_pos: pos, digit: number_at(part.clone(), pos) as u64 }
                    ))
            .take(99)
            .fold(None, |some_d, e| -> Option<IndexedDigit> {
                Some(some_d
                    .map_or(e.clone(), |d| if d.compare_in_order(&e) == Ordering::Less { e } else { d }))
            }).unwrap();

        let second_largest = self.number_parts
            .iter()
            .zip((0 as u32)..)
            .flat_map(|(part, vec_index)| (0..NUMBERS_IN_U64.min(part.ilog(10) + 1))
                .map(move |pos|
                    IndexedDigit { index: vec_index, inner_pos: pos, digit: number_at(part.clone(), pos) as u64 }
                    ))
            .filter(|d| -> bool {
                if d.index != largest_digit.index {
                    d.index > largest_digit.index
                } else {
                    d.inner_pos > largest_digit.inner_pos
                }
            })
            .fold(None, |some_d, e| -> Option<IndexedDigit> {
                Some(some_d
                    .map_or(e.clone(), |d| if d.compare_in_order(&e) == Ordering::Less { e } else { d }))
            }).unwrap();
        

        (largest_digit.digit * 10 + second_largest.digit) as u32
    }

    fn joltage_of_digitn(&self, digit: u8, previous_max_digit: Option<&IndexedDigit>) -> IndexedDigit {
        self.number_parts
            .iter()
            .zip((0 as u32)..)
            .flat_map(|(part, vec_index)| (0..NUMBERS_IN_U64.min(part.ilog(10) + 1))
                .map(move |pos|
                    IndexedDigit { index: vec_index, inner_pos: pos, digit: number_at(part.clone(), pos) as u64 }
                    ))
            .take(100 - (12 - (digit as usize)))
            .filter(|d| -> bool {
                if let Some(max_digit) = previous_max_digit.clone() {
                    if d.index != max_digit.index {
                        d.index > max_digit.index
                    } else {
                        d.inner_pos > max_digit.inner_pos
                    }
                } else { true }
            })
            .fold(None, |some_d, e| -> Option<IndexedDigit> {
                Some(some_d
                    .map_or(e.clone(), |d| if d.compare_in_order(&e) == Ordering::Less { e } else { d }))
            }).unwrap()
    }

    fn joltage_with_12_digits(self) -> u64 {
        let mut digits: Vec<IndexedDigit> = vec![];
        for n_pow in 1..13 {
            digits.push(self.joltage_of_digitn(n_pow, digits.last()));
        }

        digits
            .iter()
            .fold(0, |total, x| total * 10 + x.digit)
    }
}

fn read_rank(s: String) -> Option<Rank> {
    let mut result: Vec<u64> = vec![];

    let mut strg = s.clone();
    while strg.len() > 0 {
        let (d, e) = if (strg.len() as u32) > NUMBERS_IN_U64 {
            let (dd, ee) = strg.split_at(19);
            (dd.to_string(), ee.to_string())
        } else {
            (strg.clone(), "".to_string())
        };

        strg = e;
        let number_part = d.parse().ok()?;
        result.push(number_part);
    }

    Some(Rank { number_parts: result })
}

pub async fn answer() -> Option<()> {
    let raw_ranks = read_lines("day-03.txt").await?;
    
    let total = raw_ranks
        .iter()
        .map(|differ| read_rank(differ.to_owned()))
        .filter(|differ| differ.is_some())
        .map(|differ| differ.unwrap().largest_joltage())
        .fold(0, |total, x| total + x);

    println!("{0}", total);
    Some(())

}

pub async fn answer2() -> Option<()> {
    let raw_ranks = read_lines("day-03.txt").await?;
    
    let total = raw_ranks
        .iter()
        .map(|differ| read_rank(differ.to_owned()))
        .filter(|differ| differ.is_some())
        .map(|differ| differ.unwrap().joltage_with_12_digits())
        .fold(0, |total, x| total + x);

    println!("{0}", total);
    Some(())
}
