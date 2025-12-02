use std::{collections::HashSet, hash::RandomState};

#[derive(Debug)]
pub struct NumberRange {
    pub start: u64,
    pub end: u64,
}

fn previous_or_first_id(x: u64) -> u64 {
    if x <= 11 {
        return 11;
    }

    let base10 = x.ilog10();
    if base10 % 2 == 0 {
        return (10 as u64).pow(base10) - 1;
    }

    let half_base10 = (base10 + 1) / 2;

    let first_comp = x / ((10 as u64).pow(half_base10));
    let second_comp = x % ((10 as u64).pow(half_base10));
    if second_comp >= first_comp {
        return first_comp * ((10 as u64).pow(half_base10)) + first_comp;
    }
    if first_comp > 1 {
        let previous_comp = first_comp - 1;
        return previous_comp * ((10 as u64).pow(half_base10)) + previous_comp;
    }
    return (10 as u64).pow(base10 - 1) - 1;
}

fn next_id(x: u64) -> u64 {
    if x <= 11 {
        return 11;
    }

    let base10 = x.ilog10();
    if base10 % 2 == 0 {
        return (10 as u64).pow(base10 + 1) + (10 as u64).pow(base10 / 2);
    }

    let half_base10 = (base10 + 1) / 2;

    let first_comp = x / ((10 as u64).pow(half_base10));
    let second_comp = x % ((10 as u64).pow(half_base10));
    if second_comp <= first_comp {
        return first_comp * ((10 as u64).pow(half_base10)) + first_comp;
    }
    let next_comp = first_comp + 1;
    return next_comp * ((10 as u64).pow(half_base10)) + next_comp;
}

impl NumberRange {
    pub fn invalids(&self) -> Vec<u64> {

        let mut result: Vec<u64> = vec![];
        let mut cursor = previous_or_first_id((&self).start.clone());
        if cursor < (&self).start.clone() {
            cursor = next_id(cursor + 1);
        }

        while cursor <= (&self).end.clone() {
            result.push(cursor.clone());
            cursor = next_id(cursor + 1);
        }
        return result;
    }

    fn divisors(&self) -> Vec<u32> {
        let start_length: u32 = (|x: u64| if x == 0 { 1 } else { x.ilog10() + 1 }) ((&self).start.clone());
        let length: u32 = (|x: u64| if x == 0 { 1 } else { x.ilog10() + 1 }) ((&self).end.clone());
        if length <= 2 {
            vec![2]
        } else {
            (2..(length / 2 + 2)).chain(start_length..(length + 1)).collect()
        }
    }

    pub fn invalids_by_div(&self) -> Vec<u64> {

        let mut result: Vec<u64> = vec![];
        let mut cursor = self.divisors()
            .iter()
            .map(|d| previous_or_first_id_div((&self).start.clone(), d.clone()))
            .min().unwrap();

        while cursor <= (&self).end.clone() {
            if cursor >= (&self).start.clone() {
                result.push(cursor.clone());
            }
            cursor = self.divisors()
                .iter()
                .map(|d| next_id_div(cursor + 1, d.clone()))
                .min().unwrap();
        }
        return result;
    }
}

pub fn read_range(x: &str) -> Option<NumberRange> {
    let raw_numbers = x.split('-').collect::<Vec<&str>>();

    let s = u64::from_str_radix(raw_numbers[0], 10).ok()?;
    let e = u64::from_str_radix(raw_numbers[1], 10).ok()?;

    Some(NumberRange { start: s, end: e })
}

pub async fn answer() -> Option<()> {

    // let example = "0-10,450-1111,12-998,9941196-10054232"
    // let example = "9941196-10054232"
    // let example = "17330-35281,9967849351-9967954114,880610-895941,942-1466,117855-209809,9427633930-9427769294,1-14,311209-533855,53851-100089,104-215,33317911-33385573,42384572-42481566,43-81,87864705-87898981,258952-303177,451399530-451565394,6464564339-6464748782,1493-2439,9941196-10054232,2994-8275,6275169-6423883,20-41,384-896,2525238272-2525279908,8884-16221,968909030-969019005,686256-831649,942986-986697,1437387916-1437426347,8897636-9031809,16048379-16225280"
    //     .split(',')
    //     .map(|x| read_range(x))
    //     .filter(|x| x.is_some())
    //     .map(|x| x.unwrap().invalids_nightly())
    //     .collect::<Vec<Vec<u64>>>();
    // for i in 0..example.len() {
    //     println!("{:?}", example[i]);
    // }

    let answer = "17330-35281,9967849351-9967954114,880610-895941,942-1466,117855-209809,9427633930-9427769294,1-14,311209-533855,53851-100089,104-215,33317911-33385573,42384572-42481566,43-81,87864705-87898981,258952-303177,451399530-451565394,6464564339-6464748782,1493-2439,9941196-10054232,2994-8275,6275169-6423883,20-41,384-896,2525238272-2525279908,8884-16221,968909030-969019005,686256-831649,942986-986697,1437387916-1437426347,8897636-9031809,16048379-16225280"
        .split(',')
        .map(|x| read_range(x))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().invalids().iter().fold(0, |e, total| total + e))
        .fold(0, |e, total| total + e);
    println!("{answer:?}");

    Some(())
}

fn second_based_in_first(first: u64, number_log10: u32, divisor: u32) -> u64 {
    let mul = (number_log10 + 1) / divisor;
    (0..(divisor - 1))
        .map(|e| first * (10 as u64).pow(mul * e))
        .fold(0, |total, e| total + e)
}

fn previous_or_first_id_div(x: u64, divisor: u32) -> u64 {

    let first_number = (0..divisor)
        .map(|e| (10 as u64).pow(e))
        .fold(0, |total, e| total + e);
    if x <= first_number {
        return first_number;
    }

    let base10 = x.ilog10();
    if (base10 % divisor) != divisor - 1 {
        return (10 as u64).pow(base10 - (base10 % divisor)) - 1;
    }

    let part_base10 = base10 - base10 / divisor;

    let first_comp = x / ((10 as u64).pow(part_base10));
    let second_comp = x % ((10 as u64).pow(part_base10));
    let second_based = second_based_in_first(first_comp, base10, divisor);

    if second_comp >= second_based {
        return first_comp * ((10 as u64).pow(part_base10)) + second_based;
    }
    if first_comp > 1 {
        let previous_comp = first_comp - 1;
        return previous_comp * ((10 as u64).pow(part_base10)) + second_based_in_first(previous_comp, base10, divisor);
    }
    return (10 as u64).pow(base10 - (divisor - 1)) - 1;
}

fn next_id_div(x: u64, divisor: u32) -> u64 {
    let first_number = (0..divisor)
        .map(|e| (10 as u64).pow(e))
        .fold(0, |total, e| total + e);
    if x <= first_number {
        return first_number;
    }

    let base10 = x.ilog10();
    if base10 % divisor != divisor - 1 {
        let diff_to_base10 = (divisor - 1) - (base10 % divisor);
        let mul = (base10 + diff_to_base10 + 1) / divisor;
        return (1..(divisor + 1))
            .map(|e| (10 as u64).pow(mul * e - 1))
            .fold(0, |total, e| total + e);
    }

    let part_base10 = base10 - base10 / divisor;

    let first_comp = x / ((10 as u64).pow(part_base10));
    let second_comp = x % ((10 as u64).pow(part_base10));
    let second_based = second_based_in_first(first_comp, base10, divisor);

    if second_comp <= second_based {
        return first_comp * ((10 as u64).pow(part_base10)) + second_based;
    }
    let next_comp = first_comp + 1;
    return next_comp * ((10 as u64).pow(part_base10)) + second_based_in_first(next_comp, base10, divisor);
}

pub async fn answer2() -> Option<()> {

    // let example = "450-1111"
    // let example = "0-10,450-1111,12-998,9941196-10054232"
    // let example = "9941196-10054232"
    //     .split(',')
    //     .map(|x| read_range(x))
    //     .filter(|x| x.is_some())
    //     .map(|x| x.unwrap().invalids_by_div())
    //     .collect::<Vec<Vec<u64>>>();
    // for i in 0..example.len() {
    //     println!("{:?}", example[i]);
    // }

    let answer = (HashSet::from_iter(
        "17330-35281,9967849351-9967954114,880610-895941,942-1466,117855-209809,9427633930-9427769294,1-14,311209-533855,53851-100089,104-215,33317911-33385573,42384572-42481566,43-81,87864705-87898981,258952-303177,451399530-451565394,6464564339-6464748782,1493-2439,9941196-10054232,2994-8275,6275169-6423883,20-41,384-896,2525238272-2525279908,8884-16221,968909030-969019005,686256-831649,942986-986697,1437387916-1437426347,8897636-9031809,16048379-16225280"
        .split(',')
        .map(|x| read_range(x))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().invalids_by_div())
        .flatten()) as HashSet<u64, RandomState>)
        .iter()
        .fold(0, |e, total| total + e);
    println!("{answer:?}");

    Some(())
}
