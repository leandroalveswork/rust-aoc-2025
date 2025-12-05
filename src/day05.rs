use std::{collections::HashSet, hash::RandomState};

use crate::mass_parser::read_lines;

#[derive(Clone, Debug)]
pub struct FreshRange {
    pub start: u64,
    pub end: u64,
}

impl FreshRange {
    fn between(&self, x: &u64) -> bool {
        x.clone() >= self.start && x.clone() <= self.end
    }
}

fn read_range(line: String) -> Option<FreshRange> {
    let xs: Vec<&str> = line.split('-').collect();
    let start = xs.get(0)?.parse().ok()?;
    let end = xs.get(1)?.parse().ok()?;

    Some(FreshRange { start: start, end: end })
}

pub struct Problem {
    pub ids: HashSet<u64>,
    pub ranges: Vec<FreshRange>,
}

fn read_problem(lines: Vec<String>) -> Option<Problem> {
    let empty_index = lines.iter().zip(0..)
        .find(|(x, _idx)| x.is_empty())
        .map(|x| x.1)?;

    let ranges: Vec<FreshRange> = lines.iter().take(empty_index)
        .filter_map(|x| read_range(x.clone()))
        .collect();

    let ids: HashSet<u64, RandomState> = HashSet::from_iter(
        lines.iter().skip(empty_index + 1)
        .filter_map(|x| -> Option<u64> { x.parse().ok() })
        );

    Some(Problem { ids: ids, ranges: ranges })
}

pub async fn answer() -> Option<()> {

    let mass = read_lines("day-05.txt").await?;
    // let mass = vec![
    //     "50601-52000".to_string(),
    //     "133128-177810".to_string(),
    //     "2330-4329".to_string(),
    //     "".to_string(),
    //     "12".to_string(),
    //     "50488".to_string(),
    //     "50600".to_string(),
    //     "50601".to_string(),
    //     "177810".to_string(),
    //     "3400".to_string(),
    //     "29993878".to_string()
    // ];
    let problem = read_problem(mass)?;

    // ranges_by_ranges(&mut problem.ranges);

    let fresh_ids = problem.ids.iter().filter(|n|
        problem.ranges.iter().any(|r|
            r.between(n)))
        .count();

    println!("{fresh_ids:?}");

    Some(())
}

impl FreshRange {
    pub fn merge_with(&self, other: &FreshRange) -> Self {

        FreshRange { 
            start: self.start.min(other.start), 
            end: self.end.max(other.end)
        }
    }

    pub fn merge_with_ranges(&self, other: &mut Vec<FreshRange>) -> Self {
        let current_size = self.end - self.start;
        let mut range = self.clone();

        let other_freeze = other.clone();
        for n in 1..(other_freeze.len() + 1) {
            let other_idx = other_freeze.len() - n;
            let other_x = &other[other_idx];

            if other_x.between(&self.start) || other_x.between(&self.end) || self.between(&other_x.start) {
                range = range.merge_with(other_x);
                other.remove(other_idx);
            }
        }

        let new_size = range.end - range.start;
        if new_size != current_size {
            range.merge_with_ranges(other)
        } else {
            range
        }
    }
}

pub fn ranges_by_ranges(ranges: &mut Vec<FreshRange>) {
    let mut result: Vec<FreshRange> = vec![];

    while !ranges.is_empty() {
        let mut first = ranges.remove(0);
        first = first.merge_with_ranges(ranges);
        result.push(first);
    }

    for x in result {
        ranges.push(x);
    }
}

pub async fn answer2() -> Option<()> {

    let mass = read_lines("day-05.txt").await?;
    // let mass = vec![
    //     "50601-52000".to_string(),
    //     "1-13".to_string(),
    //     "2330-4329".to_string(),
    //     "2630-4329".to_string(),
    //     "4329-6329".to_string(),
    //     "10-17".to_string(),
    //     "".to_string(),
    //     "12".to_string(),
    // ];
    let mut problem = read_problem(mass)?;

    ranges_by_ranges(&mut problem.ranges);

    let total: u64 = problem.ranges.iter()
        .map(|x| x.end - x.start + 1)
        .sum();

    println!("{total:?}");

    Some(())
}
