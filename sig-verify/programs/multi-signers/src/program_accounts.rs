use anchor_lang::prelude::*;

#[account]
pub struct State {
  pub auth_provider: Pubkey,
}

#[account]
#[derive(Default)]
pub struct UserInfo {
  pub total_amount: u64,
}
