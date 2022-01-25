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
