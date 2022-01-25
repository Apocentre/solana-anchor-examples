use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};
use std::{
  result::Result as StdResult
};

#[error]
pub enum ErrorCode {
  #[msg("overflow")]
  Overflow,
}


macro_rules! math_wrapper {
  ($name: ident, $type: ident) => {
    #[derive(BorshSerialize, BorshDeserialize, Default, Copy, Clone, Debug)]
    pub struct $name(pub $type);

    impl $name {
      pub fn add(&self, rhs: Self) -> StdResult<Self, ProgramError> {
        match self.0.checked_add(rhs.0) {
          Some(result) => Ok($name(result)),
          None => return Err(ErrorCode::Overflow.into())
        }
      }
    }
  }
}

math_wrapper!(U64, u64);
