use std::fs::File;
use crate::types::MyResult;
use std::io::{BufRead, BufReader};


fn seeds(line: &str) -> MyResult<Vec<u32>> {
    let space = line.find(' ').unwrap();
    let seed_str = &line[space + 1..];
    let seeds: Vec<u32> = seed_str.split(' ')
        .map(|a| a.parse())
        .collect::<Result<_, _>>()?;
    return Ok(seeds);
}

enum State {
    Seeds,
    Rest
}

pub fn p1(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut state = State::Seeds;

    for line in reader.lines() {
        let line = line?;
        match &state {
            State::Seeds => {
                let seeds = seeds(line.as_ref())?;
                println!("seeds {:?}", seeds);
                state = State::Rest;
            }
            State::Rest => {}
        }
    }
    Ok(())
}