use clap::Parser;

use solver::Solver;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  day: usize,
  #[arg(short, long)]
  input: String,
}

impl read::HasFile for Args {
  fn file(&self) -> String { self.input.clone() }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let (input, flags) = read::input_with_type::<Args>(None);
  let solver: Box<dyn Solver> = match flags.day {
    // BEGIN_SOLVER_LIST
    0 => day00::Day00::try_create(input),
    // END_SOLVER_LIST
    _ => panic! {"Failed to find solver"},
  }?;
  println! {"{}", solver.part_one()?};
  println! {"{}", solver.part_two()?};
  Ok(())
}
