use anchor_lang::prelude::*;
use std::{
  convert::From,
  result::Result as StdResult
};

#[error]
pub enum ErrorCode {
  #[msg("overflow")]
  Overflow,
  #[msg("underflow")]
  Underflow
}

macro_rules! math_wrapper_type {
  ($name: ident, $type: ident) => {
    #[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone, Debug)]
    pub struct $name(pub $type);

    impl $name {
      pub fn add(&self, rhs: Self) -> StdResult<Self, ProgramError> {
        match self.0.checked_add(rhs.0) {
          Some(result) => Ok($name(result)),
          None => return Err(ErrorCode::Overflow.into())
        }
      }

      pub fn sub(&self, rhs: Self) -> StdResult<Self, ProgramError> {
        match self.0.checked_sub(rhs.0) {
          Some(result) => Ok($name(result)),
          None => return Err(ErrorCode::Underflow.into())
        }
      }
    }

    impl From<$type> for $name {
      fn from(v: $type) -> $name {
        $name(v)
      }
    }
  }
}

math_wrapper_type!(U64, u64);
math_wrapper_type!(U32, u32);

// At the moment the anchor lib on the front end and more specifically the IDL does not support
// custom types and thus we will use function and not type to accomplish the same task

macro_rules! math_wrapper {
  ($type: ident) => {
    pub fn add(lsh: $type, rhs: $type) -> StdResult<$type, ProgramError> {
      match lsh.checked_add(rhs) {
        Some(result) => Ok(result),
        None => return Err(ErrorCode::Overflow.into())
      }
    }

    pub fn sub(lsh: $type, rhs: $type) -> StdResult<$type, ProgramError> {
      match lsh.checked_sub(rhs) {
        Some(result) => Ok(result),
        None => return Err(ErrorCode::Underflow.into())
      }
    }
  }
}

math_wrapper!(u64);


// Third alternative solution is to extend the types with the custom trait
pub trait SafeMath {
  type Output;

  fn safe_add(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_sub(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
}

macro_rules! safe_math {
  ($type: ident) => {
    impl SafeMath for $type {
      type Output = $type;

      fn safe_add(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_add(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Overflow.into())
        }
      }
    
      fn safe_sub(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_sub(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Underflow.into())
        }
      }
    }
  }
}

safe_math!(u64);
