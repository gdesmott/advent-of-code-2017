extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

fn compute(input: &str) -> u32 {
    let numbers: Vec<u32> = input
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    let mut sum = 0;
    let n = numbers.len();

    for i in 0..n {
        let prev = {
            if i == 0 {
                numbers[n - 1]
            } else {
                numbers[i - 1]
            }
        };

        if numbers[i] == prev {
            sum += numbers[i]
        }
    }

    sum
}

#[derive(StructOpt)]
#[structopt(name = "day-1", about = "http://adventofcode.com/2017/day/1")]
struct Opt {
    #[structopt(help = "Input")]
    input: String,
}

fn main() {
    let opt = Opt::from_args();

    println!("{}", compute(opt.input.as_ref()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(compute("1122"), 3);
        assert_eq!(compute("1111"), 4);
        assert_eq!(compute("1234"), 0);
        assert_eq!(compute("91212129"), 9);
    }
}
