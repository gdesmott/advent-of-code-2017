use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

fn compute(input: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for row in input {
        let diff = row.iter().max().unwrap() - row.iter().min().unwrap();
        sum += diff;
    }

    sum
}


fn parse(input: &File) -> Vec<Vec<u32>> {
    let mut sheet = Vec::new();
    let reader = BufReader::new(input);

    for l in reader.lines() {
        let mut row: Vec<u32> = Vec::new();

        for elt in l.unwrap().split_whitespace() {
            row.push(elt.parse().unwrap());
        }

        sheet.push(row);
    }

    sheet
}

#[derive(StructOpt)]
#[structopt(name = "day-2", about = "http://adventofcode.com/2017/day/2")]
struct Opt {
    #[structopt(help = "Input file")]
    input: String,
}

fn main() {
    let opt = Opt::from_args();
    let f = File::open(opt.input).expect("Failed to open input file");
    let s = parse(&f);

    println!("{}", compute(&s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let s = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];

        assert_eq!(compute(&s), 18);
    }

    #[test]
    fn test_parse() {
        let f = File::open("examples/day-2/first.txt").expect("Failed to open example");

        let s = parse(&f);
        assert_eq!(compute(&s), 18);
    }
}
