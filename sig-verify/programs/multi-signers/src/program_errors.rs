use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
  #[msg("unauthorized")]
  Unauthorized,
  #[msg("unsupported token")]
  UnsupportedToken,
  #[msg("wrong token account owner")]
  WrongTokenAccountOwner,
}
