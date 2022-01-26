use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
  #[msg("unauthorized")]
  Unauthorized,
  #[msg("unsupported token")]
  UnsupportedToken
}
