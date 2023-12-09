#![feature(test)]
extern crate test;

use std::num::ParseIntError;

use solver::Solver;

pub struct Day01 {
  fuel: Vec<usize>,
}

impl Day01 {
  pub fn try_create(
    input: Box<(dyn Iterator<Item = String>)>,
  ) -> Result<Box<dyn Solver>, Box<dyn std::error::Error>> {
    let fuel = input
      .map(|s| s.parse::<usize>())
      .collect::<Result<Vec<usize>, ParseIntError>>()
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    Ok(Box::new(Day01 { fuel }))
  }
}

fn calculate_fuel(weight: &usize) -> usize { (weight / 3).checked_sub(2).unwrap_or(0) }

fn calculate_fuel_recursively(weight: &usize) -> usize {
  let mut remainder = *weight;
  let mut sum = 0;
  while remainder != 0 {
    let fuel = calculate_fuel(&remainder);
    sum += fuel;
    remainder = fuel;
  }
  sum
}

impl Solver for Day01 {
  fn part_one(&self) -> std::io::Result<String> {
    Ok(
      self
        .fuel
        .iter()
        .map(calculate_fuel)
        .sum::<usize>()
        .to_string(),
    )
  }

  fn part_two(&self) -> std::io::Result<String> {
    Ok(
      self
        .fuel
        .iter()
        .map(calculate_fuel_recursively)
        .sum::<usize>()
        .to_string(),
    )
  }
}
#[cfg(test)]
mod tests {
  use std::error::Error;

  use super::*;
  use test::Bencher;

  #[test]
  fn it_works_on_the_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../puzzles/day01/example.input")
      .lines()
      .map(String::from);

    let mut solver = Day01::try_create(Box::new(input)).unwrap();
    assert_eq! {solver.part_one()?, "34241"};
    Ok(())
  }

  #[test]
  fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../puzzles/day01/example.input")
      .lines()
      .map(String::from);

    let mut solver = Day01::try_create(Box::new(input)).unwrap();
    assert_eq! {solver.part_two()?, "51316"};
    Ok(())
  }

  #[test]
  fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../puzzles/day01/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day01::try_create(Box::new(input)).unwrap();
    assert_eq! {solver.part_one()?, "3416712"};
    assert_eq! {solver.part_two()?, "5122170"};
    Ok(())
  }

  #[bench]
  fn bench_parse(b: &mut Bencher) {
    b.iter(|| {
      let input = include_str!("../../puzzles/day01/joshua.input")
        .lines()
        .map(String::from);

      let _solver = Day01::try_create(Box::new(input)).unwrap();
    });
  }

  #[bench]
  fn bench_one(b: &mut Bencher) {
    let input = include_str!("../../puzzles/day01/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day01::try_create(Box::new(input)).unwrap();

    b.iter(|| solver.part_one());
  }

  #[bench]
  fn bench_two(b: &mut Bencher) {
    let input = include_str!("../../puzzles/day01/joshua.input")
      .lines()
      .map(String::from);

    let mut solver = Day01::try_create(Box::new(input)).unwrap();

    b.iter(|| solver.part_two());
  }
}
