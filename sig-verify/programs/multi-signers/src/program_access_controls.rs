use anchor_lang::prelude::*;
use super::{
  program_accounts::{State},
  program_errors::{ErrorCode},
};

pub fn authenticate(auth_provider: &Signer, state: &State) -> ProgramResult {
  // tx should be signed by both the sender and the auth provider
  if auth_provider.unsigned_key() != &state.auth_provider {
    return Err(ErrorCode::Unauthorized.into())
  }

  Ok(())
}

// make sure that the token account user provided with the instruction is the one that
// the program account supports
pub fn check_token(user_token_account: &Pubkey, purchase_token: &Pubkey) -> ProgramResult {
  if user_token_account != purchase_token {
    return Err(ErrorCode::UnsupportedToken.into())
  }

  Ok(())
}
