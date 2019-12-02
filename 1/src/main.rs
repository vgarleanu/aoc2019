use std::fs::File;
use std::io::prelude::*;

fn calc(x: i32) -> i32 {
    let x = ((x / 3) as f64).floor() as i32 - 2;
    if x > 0 {
        x + calc(x)
    } else {
        0
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let fuel = contents
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .map(calc)
        .filter(|x| *x > 0)
        .sum::<i32>();

    println!("Fuel requirement is: {}", fuel);
    Ok(())
}
