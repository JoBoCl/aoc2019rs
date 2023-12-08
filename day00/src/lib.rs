#![feature(test)]
extern crate test;

use solver::Solver;

pub struct Day00 {}

impl Day00 {
  pub fn try_create(
    _input: Box<(dyn Iterator<Item = String>)>,
  ) -> Result<Box<dyn Solver>, Box<dyn std::error::Error>> {
    Ok(Box::new(Day00 {}))
  }
}

impl Solver for Day00 {
  fn part_one(&mut self) -> std::io::Result<String> {
    Err(std::io::Error::new(
      std::io::ErrorKind::Other,
      "Not Implemented yet",
    ))
  }

  fn part_two(&mut self) -> std::io::Result<String> {
    Err(std::io::Error::new(
      std::io::ErrorKind::Other,
      "Not Implemented yet",
    ))
  }
}
#[cfg(test)]
mod tests {
  use std::error::Error;

  use super::*;
  use test::Bencher;

  #[test]
  fn it_works_on_the_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../puzzles/day00/example.input")
      .lines()
      .map(String::from);

    let mut solver = Day00::try_create(Box::new(input)).unwrap();
    assert! {solver.part_one().is_err()};
    Ok(())
  }

  #[test]
  fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../puzzles/day00/example.input")
      .lines()
      .map(String::from);

    let mut solver = Day00::try_create(Box::new(input)).unwrap();
    assert! {solver.part_two().is_err()};
    Ok(())
  }

  #[test]
  fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../puzzles/day00/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day00::try_create(Box::new(input)).unwrap();
    assert! {solver.part_one().is_err()};
    assert! {solver.part_two().is_err()};
    Ok(())
  }

  #[bench]
  fn bench_parse(b: &mut Bencher) {
    b.iter(|| {
      let input = include_str!("../../puzzles/day00/joshua.input")
        .lines()
        .map(String::from);

      let _solver = Day00::try_create(Box::new(input)).unwrap();
    });
  }

  #[bench]
  fn bench_one(b: &mut Bencher) {
    let input = include_str!("../../puzzles/day00/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day00::try_create(Box::new(input)).unwrap();

    b.iter(|| solver.part_one());
  }

  #[bench]
  fn bench_two(b: &mut Bencher) {
    let input = include_str!("../../puzzles/day00/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day00::try_create(Box::new(input)).unwrap();

    b.iter(|| solver.part_two());
  }
}
