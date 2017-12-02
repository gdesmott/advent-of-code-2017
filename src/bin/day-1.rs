extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

extern crate modulo;
use modulo::Mod;

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

fn compute2(input: &str) -> u32 {
    let numbers: Vec<u32> = input
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    let mut sum = 0;
    let n = numbers.len();

    // list has an even number of elements
    assert_eq!(n % 2, 0);

    for i in 0..n {
        let prev = {
            let delta = i as i32 - n as i32 / 2;
            // We need the mathematical modulo while the '%' operator is the remainder
            let j = delta.modulo(n as i32);
            numbers[j as usize]
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
    #[structopt(short = "s", help = "Second part")]
    second_part: bool,
}

fn main() {
    let opt = Opt::from_args();
    let result = match opt.second_part {
        true => compute2(opt.input.as_ref()),
        false => compute(opt.input.as_ref()),
    };

    println!("{}", result);
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

    #[test]
    fn examples2() {
        assert_eq!(compute2("1212"), 6);
        assert_eq!(compute2("1221"), 0);
        assert_eq!(compute2("123425"), 4);
        assert_eq!(compute2("123123"), 12);
        assert_eq!(compute2("12131415"), 4);
    }
}
