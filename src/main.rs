mod types;
mod day1;
mod day5;
mod day2;
mod day3;
mod day4;

use types::MyResult;

fn main() -> MyResult<()> {
    println!("\n-- Run --",);
    day1::p1("./input/input1.txt")?;
    day1::p2("./input/input1.txt")?;
    day2::p1("./input/input2.txt")?;
    day2::p2("./input/input2.txt")?;
    day3::p1("./input/input3.txt")?;
    day3::p2("./input/input3.txt")?;
    day4::p1("./input/input4.txt")?;
    day4::p2("./input/input4.txt")?;
    day5::p1("./input/demo5.txt")?;
    Ok(())
}
