use anchor_lang::prelude::*;

#[account]
pub struct State {
  pub auth_provider: Pubkey,
  // Custom defined types like U64 currently cannot work on the js lib of anchor
  pub total_raised: u64,
}

#[account]
#[derive(Default)]
pub struct UserInfo {
  pub total_amount: u64,
}
