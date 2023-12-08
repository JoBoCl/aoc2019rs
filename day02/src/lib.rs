#![feature(test)]
extern crate test;

use std::collections::HashMap;

use solver::Solver;

pub struct Day02 {
  intcode: intcode::IntCode,
}

impl Day02 {
  pub fn try_create(
    mut input: Box<(dyn Iterator<Item = String>)>,
  ) -> Result<Box<dyn Solver>, Box<dyn std::error::Error>> {
    let intcode = intcode::IntCode::try_from(&input.next().unwrap())?;
    Ok(Box::new(Day02 { intcode }))
  }
}

impl Solver for Day02 {
  fn part_one(&mut self) -> std::io::Result<String> {
    let mut intcode = self.intcode.clone();
    let mut mutations: HashMap<usize, i32> = HashMap::new();
    mutations.insert(1, 12);
    mutations.insert(2, 2);
    intcode.mutate(mutations);
    while !intcode.finished() {
      let _ = intcode.execute().map_err(|e| std::io::Error::other(e))?;
    }
    Ok(intcode.register_zero().to_string())
  }

  fn part_two(&mut self) -> std::io::Result<String> {
    for noun in 0..100 {
      for verb in 0..100 {
        let mut intcode = self.intcode.clone();
        let mut mutations: HashMap<usize, i32> = HashMap::new();
        mutations.insert(1, noun);
        mutations.insert(2, verb);
        intcode.mutate(mutations);
        while !intcode.finished() {
          let _ = intcode.execute().map_err(|e| std::io::Error::other(e))?;
        }

        if intcode.register_zero() == 19690720 {
          return Ok((100 * noun + verb).to_string());
        }
      }
    }
    Err(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "Could not find noun-verb pair",
    ))
  }
}
#[cfg(test)]
mod tests {
  use std::error::Error;

  use super::*;
  use test::Bencher;

  #[test]
  fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../puzzles/day02/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day02::try_create(Box::new(input)).unwrap();
    assert_eq! {solver.part_one()?, "6327510"};
    assert_eq! {solver.part_two()?, "4112"};
    Ok(())
  }

  #[bench]
  fn bench_parse(b: &mut Bencher) {
    b.iter(|| {
      let input = include_str!("../../puzzles/day02/joshua.input")
        .lines()
        .map(String::from);

      let _solver = Day02::try_create(Box::new(input)).unwrap();
    });
  }

  #[bench]
  fn bench_one(b: &mut Bencher) {
    let input = include_str!("../../puzzles/day02/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day02::try_create(Box::new(input)).unwrap();

    b.iter(|| solver.part_one());
  }

  #[bench]
  fn bench_two(b: &mut Bencher) {
    let input = include_str!("../../puzzles/day02/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day02::try_create(Box::new(input)).unwrap();

    b.iter(|| solver.part_two());
  }
}
