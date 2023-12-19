use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use crate::types::MyResult;

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

pub fn p1(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
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

pub fn p2(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
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
