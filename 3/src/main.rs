use std::collections::HashSet;
use std::convert::Into;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Eq)]
struct Coord(i32, i32, u32);

impl std::cmp::PartialEq for Coord {
    #[inline]
    fn eq(&self, other: &Coord) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl std::hash::Hash for Coord {
    fn hash<H: std::hash::Hasher>(&self, into: &mut H) {
        self.0.hash(into);
        self.1.hash(into);
    }
}

impl Coord {
    fn new() -> Self {
        Self(0, 0, 0)
    }

    fn manhattan_dist(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

impl Into<(i32, i32)> for Coord {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

#[derive(Debug)]
enum Step {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

impl Step {
    fn unwrap(&self) -> i32 {
        match self {
            Step::U(x) => *x,
            Step::D(x) => *x,
            Step::L(x) => *x,
            Step::R(x) => *x,
        }
    }
}

impl From<String> for Step {
    fn from(mut x: String) -> Step {
        let step = x.remove(0);
        let steps = x.parse::<i32>().unwrap();
        match step {
            'U' => Step::U(steps),
            'D' => Step::D(steps),
            'L' => Step::L(steps),
            'R' => Step::R(steps),
            _ => panic!(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut paths: Vec<HashSet<Coord>> = Vec::new();
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let wires = contents
        .split('\n')
        .filter(|y| y.len() > 1)
        .map(|x| {
            x.split(',')
                .filter(|y| y.len() > 1)
                .map(|y| Step::from(y.to_owned()))
                .collect::<Vec<Step>>()
        })
        .collect::<Vec<Vec<Step>>>();

    let mut index = 0;
    for wire in wires {
        let mut coord = Coord::new();
        paths.push(HashSet::new());
        for step in wire {
            for step_count in 0..step.unwrap() {
                match step {
                    Step::U(_) => coord.1 -= 1,
                    Step::D(_) => coord.1 += 1,
                    Step::L(_) => coord.0 -= 1,
                    Step::R(_) => coord.0 += 1,
                }
                coord.2 = step_count as u32;
                paths[index].insert(coord.clone());
            }
        }
        index += 1;
    }

    let mut intersections = paths[0]
        .intersection(&paths[1])
        .map(|x| (x.manhattan_dist(), x.2))
        .collect::<Vec<_>>();
    intersections.sort_by(|a, b| b.0.cmp(&a.0));

    println!("{:?}", intersections.pop().unwrap());

    Ok(())
}
