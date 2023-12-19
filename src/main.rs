use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
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

fn day1() -> MyResult<()> {
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

    let day1p1 = day1_do(&value_map_p1)?;

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

    let day1p2 = day1_do(&value_map_p2)?;

    println!("day1p1: {:?}", day1p1);
    println!("day1p2: {:?}", day1p2);
    Ok(())
}

fn day1_do(value_map: &[(&str, i32)]) -> MyResult<i32> {
    let file = File::open("input1.txt")?;
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

#[derive(Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl Round {
    fn add_round(mut self, round: MyResult<(&str, i32)>) -> MyResult<Round> {
        match round? {
            ("red", red) => self.red += red,
            ("green", green) => self.green += green,
            ("blue", blue) => self.blue += blue,
            _ => panic!("not a good day"),
        };
        Ok(self)
    }
    fn new(red: i32, green: i32, blue: i32) -> Round {
        Round {
            red: red,
            green: green,
            blue: blue,
        }
    }
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

fn day2p1() -> MyResult<()> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);
    let out = reader
        .lines()
        .map(|line| -> MyResult<_> {
            let line = line?;
            let colon = line.find(':').unwrap();
            let game: i32 = line[5..colon].parse().unwrap();

            let count = line[colon + 1..]
                .split(";")
                .map(|part| -> MyResult<Round> {
                    Ok(part
                        .split(",")
                        .map(|round| -> MyResult<_> {
                            let round = round.trim();
                            let space = round.find(' ').ok_or("no space")?;
                            Ok((&round[space + 1..], round[0..space].parse()?))
                        })
                        .try_fold(Round::new(0, 0, 0), Round::add_round)?)
                })
                .filter(|row| match row {
                    Ok(Round { red, green, blue }) => *red > 12 || *green > 13 || *blue > 14,
                    Err(_) => true,
                })
                .try_fold(0, |count, round| round.map(|_| count + 1))?;
            Ok((game, count))
        })
        .try_fold(0, |sum, res| {
            res.map(|(game, count)| if count > 0 { sum } else { sum + game })
        })?;
    println!("day2p1: {:?}", out);
    Ok(())
}

fn day2p2() -> MyResult<()> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);
    let out = reader
        .lines()
        .map(|line| -> MyResult<_> {
            let line = line?;
            let colon = line.find(':').ok_or("no colon")?;
            let rounds = &line[colon + 1..];
            let res = rounds
                .split(";")
                .map(|part| -> MyResult<Round> {
                    Ok(part
                        .split(",")
                        .map(|round| -> MyResult<_> {
                            let round = round.trim();
                            let space = round.find(' ').ok_or("no space")?;
                            Ok((&round[space + 1..], round[0..space].parse()?))
                        })
                        .try_fold(Round::new(0, 0, 0), Round::add_round)?)
                })
                .try_fold(Round::new(0, 0, 0), |max, round| {
                    round.map(|Round { red, green, blue }| Round {
                        red: cmp::max(red, max.red),
                        green: cmp::max(green, max.green),
                        blue: cmp::max(blue, max.blue),
                    })
                })?;
            Ok(res.power())
        })
        .try_fold(0, |sum, res| res.map(|round| sum + round))?;
    println!("day2p2: {:?}", out);
    Ok(())
}
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

fn day3p1() -> MyResult<()> {
    let file = File::open("input3.txt")?;
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

fn day3p2() -> MyResult<()> {
    let file = File::open("input3.txt")?;
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

fn numbers_in(chars: &str) -> MyResult<Vec<u32>> {
    Ok(chars
        .split(' ')
        .filter(|l| l.len() != 0)
        .map(|a| Ok(a.parse()?))
        .collect::<MyResult<Vec<u32>>>()?)
}

fn winners_in(winners: &Vec<u32>, drawn: &Vec<u32>) -> u32 {
    let count = drawn
        .iter()
        .filter(|d| winners.iter().find(|w| w == d).is_some())
        .count();
    if count > 0 {
        2u32.pow((count - 1).try_into().unwrap())
    } else {
        0
    }
}

fn day4p1() -> MyResult<()> {
    let file = File::open("input4.txt")?;
    let reader = BufReader::new(file);
    let sum = reader.lines().try_fold(0u32, |sum, line| -> MyResult<_> {
        let line = line?;
        let colon = line.find(':').ok_or("Colon?")?;
        let pipe = line.find('|').ok_or("Pipe?")?;
        let winners = numbers_in(&line[colon + 1..pipe])?;
        let drawn = numbers_in(&line[pipe + 1..])?;
        let res = winners_in(&winners, &drawn);
        Ok(sum + res)
    })?;
    println!("day4p1 {}", sum);
    Ok(())
}

fn winners_in_p2(winners: &Vec<u32>, drawn: &Vec<u32>) -> usize {
    drawn
        .iter()
        .filter(|d| winners.iter().find(|w| w == d).is_some())
        .count()
}

fn day4p2() -> MyResult<()> {
    let file = File::open("input4.txt")?;
    let reader = BufReader::new(file);
    let (backlog, _) = reader.lines().try_fold(
        (vec![], 0),
        |(mut backlog, i): (Vec<usize>, usize), line| -> MyResult<_> {
            let line = line?;
            let colon = line.find(':').ok_or("Colon?")?;
            let pipe = line.find('|').ok_or("Pipe?")?;
            let winners = numbers_in(&line[colon + 1..pipe])?;
            let drawn = numbers_in(&line[pipe + 1..])?;
            let res = winners_in_p2(&winners, &drawn);
            backlog.resize(cmp::max(i + 1 + res, backlog.len()), 0);
            backlog[i] += 1;
            let win = backlog[i];
            if res > 0 {
                for j in (i + 1)..(i + 1 + res) {
                    backlog[j] += win;
                }
            }
            Ok((backlog, i + 1))
        },
    )?;
    println!("day4p2 {}", backlog.iter().fold(0, |a, b| a + b));
    Ok(())
}

fn main() -> MyResult<()> {
    println!("\n-- Run --",);
    day1()?;
    day2p1()?;
    day2p2()?;
    day3p1()?;
    day3p2()?;
    day4p1()?;
    day4p2()?;
    Ok(())
}
