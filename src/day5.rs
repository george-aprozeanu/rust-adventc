use crate::types::MyResult;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Pair {
    from: u32,
    to: u32,
    value: u32,
}

impl Pair {
    fn from(vec: &Vec<&str>) -> MyResult<Pair> {
        Ok(Pair {
            from: vec[0].parse()?,
            to: vec[1].parse()?,
            value: vec[2].parse()?,
        })
    }
}

#[derive(Debug)]
struct InputMap {
    from: String,
    to: String,
    pairs: Vec<Pair>,
}

impl InputMap {
    fn read(lines: &mut impl Iterator<Item = io::Result<String>>) -> MyResult<Option<InputMap>> {
        if let Some(title_line) = lines.next() {
            let title_line = title_line?;
            let from_sep = title_line.find("-to-").ok_or("'-to-' ?")?;
            let end_sep = title_line.find(" map:").ok_or("' map:' ?")?;
            let from = String::from(&title_line[0..from_sep]);
            let to = String::from(&title_line[from_sep + 4..end_sep]);
            let mut pairs: Vec<Pair> = vec![];
            loop {
                if let Some(line) = lines.next() {
                    let line = line?;
                    if line.len() == 0 {
                        break;
                    }
                    let parts: Vec<&str> = line.splitn(3, ' ').collect();
                    pairs.push(Pair::from(&parts)?);
                } else {
                    break;
                }
            }
            Ok(Some(InputMap { from, to, pairs }))
        } else {
            Ok(None)
        }
    }
}

fn seeds(line: &str) -> MyResult<Vec<u32>> {
    let space = line.find(' ').ok_or("space?")?;
    let seed_str = &line[space + 1..];
    let seeds: Vec<u32> = seed_str
        .split(' ')
        .map(|a| a.parse())
        .collect::<Result<_, _>>()?;
    return Ok(seeds);
}

pub fn p1(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let line = lines.next().ok_or("first?")??;
    let seeds = seeds(line.as_ref());
    lines.next().ok_or("next?")??;
    loop {
        if let Some(map) = InputMap::read(&mut lines)? {
            println!("map {:?}", map);
        } else {
            break;
        }
    }
    Ok(())
}
