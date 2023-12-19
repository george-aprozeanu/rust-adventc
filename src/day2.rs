use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::types::MyResult;

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

pub fn p1(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
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

pub fn p2(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
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