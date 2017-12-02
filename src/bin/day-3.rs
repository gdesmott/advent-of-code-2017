extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

#[derive(Clone, Copy, Debug)]
enum Orientation {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Debug)]
struct Axis {
    orientation: Orientation,
    numbers: Vec<u32>,
}

impl Axis {
    fn new(orientation: Orientation) -> Axis {
        Axis {
            orientation: orientation,
            numbers: Vec::new(),
        }
    }
}

struct Grid {
    north: Axis,
    south: Axis,
    east: Axis,
    west: Axis,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            north: Axis::new(Orientation::NORTH),
            south: Axis::new(Orientation::SOUTH),
            east: Axis::new(Orientation::EAST),
            west: Axis::new(Orientation::WEST),
        }
    }

    fn next_axis(&mut self, current: &Orientation) -> &mut Axis {
        match *current {
            Orientation::NORTH => &mut self.west,
            Orientation::WEST => &mut self.south,
            Orientation::SOUTH => &mut self.east,
            Orientation::EAST => &mut self.north,
        }
    }

    // Generate the values on the axises until we exceed @n,
    // then return the two axises closest to it
    fn closet_axises(&mut self, n: u32) -> (&Axis, &Axis) {
        let mut i = 1;
        let mut inc = 1;
        let mut current = Orientation::SOUTH;

        while i < n {
            let axis = self.next_axis(&current);
            current = axis.orientation;
            let prev = axis.numbers.last().unwrap_or(&1).clone();
            i = prev + inc;
            axis.numbers.push(i);
            inc += 2;
        }

        match current {
            Orientation::NORTH => (&self.north, &self.east),
            Orientation::EAST => (&self.south, &self.west),
            Orientation::SOUTH => (&self.south, &self.east),
            Orientation::WEST => (&self.north, &self.west),
        }
    }
}

fn square_steps(start: u32) -> u32 {
    if start == 1 {
        return 0;
    }

    let mut grid = Grid::new();

    let (a1, a2) = grid.closet_axises(start);
    let (closest, distance) = [a1, a2]
        .into_iter()
        .map(|a| {
            (
                *a,
                (*a.numbers.last().unwrap() as i32 - start as i32).abs() as u32,
            )
        })
        .min_by_key(|x| x.1)
        .unwrap();

    distance + closest.numbers.len() as u32
}

#[derive(StructOpt)]
#[structopt(name = "day-3", about = "http://adventofcode.com/2017/day/3")]
struct Opt {
    #[structopt(help = "Input")]
    input: u32,
}

fn main() {
    let opt = Opt::from_args();

    println!("{}", square_steps(opt.input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(square_steps(1), 0);
        assert_eq!(square_steps(12), 3);
        assert_eq!(square_steps(23), 2);
        assert_eq!(square_steps(1024), 31);
    }
}
