#![feature(test)]
extern crate test;

use std::{collections::HashMap, num::ParseIntError};

pub fn add(left: usize, right: usize) -> usize { left + right }

#[derive(Debug, Clone)]
pub enum Op {
  Add {
    l_arg: Arg,
    r_arg: Arg,
    out_arg: Arg,
  },
  Mul {
    l_arg: Arg,
    r_arg: Arg,
    out_arg: Arg,
  },
  Stop {},
}

#[derive(Debug, Clone, Copy)]
pub enum Arg {
  Register { address: usize },
  Constant { value: i32 },
}

#[derive(Debug, Clone)]
pub struct IntCode {
  program: Vec<i32>,
  instruction_pointer: usize,
}

impl TryFrom<&String> for IntCode {
  type Error = std::io::Error;
  fn try_from(input: &String) -> Result<Self, Self::Error> {
    let program = input
      .split(',')
      .map(|s| s.parse::<i32>())
      .collect::<Result<Vec<i32>, ParseIntError>>()
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    Ok(IntCode {
      program,
      instruction_pointer: 0,
    })
  }
}

#[derive(Debug)]
pub enum OperationDecodeError {
  InvalidInstruction {
    seen_value: i32,
  },
  EndOfInstructions {
    current_instruction: usize,
    needed: usize,
  },
}

#[derive(Debug)]
pub enum OperationExecuteError {
  InvalidRegisterError { address: usize },
  InvalidOutputError { arg: Arg },
  DecodeError { cause: OperationDecodeError },
}

impl std::error::Error for OperationExecuteError {}

impl std::fmt::Display for OperationExecuteError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{self:?}"))
  }
}

impl IntCode {
  pub fn current_instruction(&self) -> Result<Op, OperationDecodeError> {
    match self.program.get(self.instruction_pointer) {
      Some(1) => {
        return if self.program.len() < self.instruction_pointer + 3 {
          Err(OperationDecodeError::EndOfInstructions {
            current_instruction: self.instruction_pointer,
            needed: 3,
          })
        } else {
          Ok(Op::Add {
            l_arg: Arg::Register {
              address: self.program[self.instruction_pointer + 1] as usize,
            },
            r_arg: Arg::Register {
              address: self.program[self.instruction_pointer + 2] as usize,
            },
            out_arg: Arg::Register {
              address: self.program[self.instruction_pointer + 3] as usize,
            },
          })
        }
      }
      Some(2) => {
        return if self.program.len() < self.instruction_pointer + 3 {
          Err(OperationDecodeError::EndOfInstructions {
            current_instruction: self.instruction_pointer,
            needed: 3,
          })
        } else {
          Ok(Op::Mul {
            l_arg: Arg::Register {
              address: self.program[self.instruction_pointer + 1] as usize,
            },
            r_arg: Arg::Register {
              address: self.program[self.instruction_pointer + 2] as usize,
            },
            out_arg: Arg::Register {
              address: self.program[self.instruction_pointer + 3] as usize,
            },
          })
        }
      }
      Some(99) => todo!(),
      Some(v) => Err(OperationDecodeError::InvalidInstruction { seen_value: *v }),
      None => Err(OperationDecodeError::EndOfInstructions {
        current_instruction: self.instruction_pointer,
        needed: 1,
      }),
    }
  }

  fn value<'a>(&'a self, arg: &'a Arg) -> Result<&'a i32, OperationExecuteError> {
    match arg {
      Arg::Register { address } => self
        .program
        .get(*address)
        .ok_or_else(|| OperationExecuteError::InvalidRegisterError { address: *address }),
      Arg::Constant { value } => Ok(value),
    }
  }

  pub fn operation_size(&self, op: Op) -> usize {
    match op {
      Op::Add { .. } => 4,
      Op::Mul { .. } => 4,
      Op::Stop {} => 0,
    }
  }

  pub fn mutate(&mut self, mutations: HashMap<usize, i32>) {
    for (k, v) in mutations {
      self.program[k] = v;
    }
  }

  pub fn finished(&self) -> bool { self.program[self.instruction_pointer] == 99 }

  pub fn execute(&mut self) -> Result<bool, OperationExecuteError> {
    let op = self
      .current_instruction()
      .map_err(|e| OperationExecuteError::DecodeError { cause: e })?;
    let finished = match op {
      Op::Add {
        l_arg,
        r_arg,
        out_arg,
      } => {
        let l = self.value(&l_arg)?;
        let r = self.value(&r_arg)?;
        match out_arg {
          Arg::Register { address } if address < self.program.len() => {
            self.program[address] = l + r;
            Ok(false)
          }
          _ => return Err(OperationExecuteError::InvalidOutputError { arg: out_arg }),
        }
      }
      Op::Mul {
        l_arg,
        r_arg,
        out_arg,
      } => {
        let l = self.value(&l_arg)?;
        let r = self.value(&r_arg)?;
        match out_arg {
          Arg::Register { address } if address < self.program.len() => {
            self.program[address] = l * r;
            Ok(false)
          }
          _ => return Err(OperationExecuteError::InvalidOutputError { arg: out_arg }),
        }
      }
      Op::Stop {} => Ok(true),
    };
    self.instruction_pointer += self.operation_size(op);
    finished
  }

  pub fn register_zero(&self) -> i32 { self.program[0] }
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! simple_tests {
    ($($name:ident: $value:expr,)*) => {
      mod tests {
      $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            let mut intcode = super::IntCode::try_from(&input.to_string()).unwrap();
            while !intcode.finished() {
              let _ = intcode.execute().map_err(|e| std::io::Error::other(e)).unwrap();
            }
            assert_eq!(intcode.register_zero(), expected);
        }


      )*
      }

      mod benches {
      use test::Bencher;
      $(
        #[bench] fn $name (b: &mut Bencher) {
            let (input, _) = $value;
            let ic = super::IntCode::try_from(&input.to_string()).unwrap();
            b.iter(||  {
              let mut intcode = ic.clone();
            while !intcode.finished() {
              let _ = intcode.execute().map_err(|e| std::io::Error::other(e)).unwrap();
            }
          });
        }
      )*
      }
    }
    }
  simple_tests! {
    day_02_1: ("1,9,10,3,2,3,11,0,99,30,40,50", 3500),
    day_02_2: ("1,0,0,0,99", 2),
    day_02_3: ("2,3,0,3,99", 2),
  }
}
