use crate::types::MyResult;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn first(value_map: &[(&str, i32)], line: &str) -> MyResult<i32> {
    value_map
        .iter()
        .map(|(name, value)| (*value, line.find(name)))
        .fold(None::<(i32, usize)>, |acc, (val, pos)| {
            acc.and_then(|acc| {
                pos.map(|pos| if acc.1 < pos { acc } else { (val, pos) })
                    .or(Some(acc))
            })
                .or_else(|| pos.map(|pos| (val, pos)))
        })
        .map(|(value, _)| value)
        .ok_or("No first".into())
}

fn last(value_map: &[(&str, i32)], line: &str) -> MyResult<i32> {
    value_map
        .iter()
        .map(|(name, value)| (*value, line.rfind(name)))
        .fold(None::<(i32, usize)>, |acc, (val, pos)| {
            acc.and_then(|acc| {
                pos.map(|pos| if acc.1 > pos { acc } else { (val, pos) })
                    .or(Some(acc))
            })
                .or_else(|| pos.map(|pos| (val, pos)))
        })
        .map(|(value, _)| value)
        .ok_or("No first".into())
}

fn day1_do(file: &str, value_map: &[(&str, i32)]) -> MyResult<i32> {
    let file = File::open(file)?;
    BufReader::new(file)
        .lines()
        .map(|result| -> MyResult<i32> {
            let line = result?;
            let first = first(value_map, &line)?;
            let last = last(value_map, &line)?;
            Ok(first * 10 + last)
        })
        .try_fold(0, |a, b| Ok(a + b?))
}


pub fn p1(file: &str) -> MyResult<()> {
    let value_map_p1 = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let day1p1 = day1_do(file, &value_map_p1)?;
    println!("day1p1: {:?}", day1p1);


    Ok(())
}

pub fn p2(file: &str) -> MyResult<()> {
    let value_map_p2 = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let day1p2 = day1_do(file, &value_map_p2)?;

    println!("day1p2: {:?}", day1p2);
    Ok(())
}

