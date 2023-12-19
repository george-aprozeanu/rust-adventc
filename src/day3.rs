use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::types::MyResult;

#[derive(Debug)]
struct Part {
    start: i32,
    end: i32,
    value: u32,
}

#[derive(Debug)]
struct Line {
    parts: Vec<Part>,
    symbols: Vec<i32>,
}

impl Line {
    fn find_parts(&self, symbols: &[&i32]) -> u32 {
        self.parts
            .iter()
            .filter(|part| {
                let rest = symbols
                    .iter()
                    .find(|symbol| part.start - 1 <= ***symbol && ***symbol <= part.end + 1)
                    .is_some();
                rest
            })
            .map(|a| a.value)
            .sum()
    }
}

impl Line {
    fn from_str(chars: &str, symbol_char: Option<char>) -> MyResult<Line> {
        let (line, _) = chars.char_indices().try_fold(
            (
                Line {
                    parts: vec![],
                    symbols: vec![],
                },
                None::<usize>,
            ),
            |(mut line, mut start_part), (index, char)| -> MyResult<_> {
                if char >= '0' && char <= '9' {
                    start_part = start_part.or_else(|| Some(index))
                } else {
                    if let Some(start) = start_part {
                        line.parts.push(Part {
                            start: start.try_into()?,
                            end: (index - 1).try_into()?,
                            value: chars[start..index].parse()?,
                        })
                    }
                    start_part = None;
                    let is_symbol = symbol_char
                        .map(|symbol_char| char == symbol_char)
                        .or_else(|| Some(char != '.'))
                        .unwrap();
                    if is_symbol {
                        line.symbols.push(index.try_into()?)
                    }
                }
                if index == chars.len() - 1 {
                    if let Some(start) = start_part {
                        line.parts.push(Part {
                            start: (start).try_into()?,
                            end: chars.len().try_into()?,
                            value: chars[start..].parse()?,
                        });
                    }
                }
                Ok((line, start_part))
            },
        )?;
        Ok(line)
    }
}

fn sum_line_p1(prev: Option<&Line>, curr: &Line, next: Option<&Line>) -> MyResult<u32> {
    let empty = vec![];
    let curr_symbols: &Vec<i32> = &curr.symbols;
    let prev_symbols = if let Some(prev) = prev {
        &prev.symbols
    } else {
        &empty
    };
    let next_symbols = if let Some(next) = next {
        &next.symbols
    } else {
        &empty
    };
    let symbols: Vec<&i32> = prev_symbols
        .into_iter()
        .chain(curr_symbols.into_iter())
        .chain(next_symbols.into_iter())
        .collect();
    Ok(curr.find_parts(&symbols[..]))
}

pub fn p1(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let (out, prev, curr) = reader
        .lines()
        .map(|chars| -> MyResult<_> { Ok(Line::from_str(&chars?, None)?) })
        .try_fold(
            (0, None, None),
            |(sum, prev, curr): (u32, Option<Line>, Option<Line>), next| -> MyResult<_> {
                let next = Some(next?);
                let line_sum = curr.as_ref().map_or(Ok(0), |curr| sum_line_p1(prev.as_ref(), curr, next.as_ref()))?;
                Ok((sum + line_sum, curr, next))
            },
        )?;
    let res = out
        + curr.map_or(Ok(0), |curr| -> MyResult<u32> {
            Ok(sum_line_p1(prev.as_ref(), &curr, None)?)
        })?;
    println!("day3p1 {}", res);
    Ok(())
}

fn part_matches(i: i32) -> impl Fn(&&Part) -> bool {
    move |a: &&Part| a.start - 1 <= i && i <= a.end + 1
}

fn sum_line_p2(prev: Option<&Line>, curr: &Line, next: Option<&Line>) -> u32 {
    let sum: u32 = curr
        .symbols
        .iter()
        .map(|i| {
            let empty: Vec<u32> = vec![];
            let prev_count: Vec<u32> = prev.map_or(empty.clone(), |a| {
                a.parts
                    .iter()
                    .filter(part_matches(*i))
                    .map(|p| p.value)
                    .collect()
            });
            let next_count: Vec<u32> = next.map_or(empty, |a| {
                a.parts
                    .iter()
                    .filter(part_matches(*i))
                    .map(|p| p.value)
                    .collect()
            });
            let curr_count: Vec<u32> = curr
                .parts
                .iter()
                .filter(part_matches(*i))
                .map(|p| p.value)
                .collect();
            [prev_count, next_count, curr_count].concat()
        })
        .filter(|v| v.len() >= 2)
        .map(|v| v.iter().fold(1, |a, b| a * b))
        .fold(0, |a, b| a + b);
    sum
}

pub fn p2(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let (out, prev, curr) = reader
        .lines()
        .map(|chars| -> MyResult<_> { Ok(Line::from_str(&chars?, Some('*'))?) })
        .try_fold(
            (0, None, None),
            |(sum, prev, curr): (u32, Option<Line>, Option<Line>), next| -> MyResult<_> {
                let next = Some(next?);
                let line_sum = curr
                    .as_ref()
                    .map_or(0, |curr| sum_line_p2(prev.as_ref(), curr, next.as_ref()));
                Ok((sum + line_sum, curr, next))
            },
        )?;
    let res = out + curr.map_or(0, |curr| sum_line_p2(prev.as_ref(), &curr, None));
    println!("day3p2 {:?}", res);
    Ok(())
}
