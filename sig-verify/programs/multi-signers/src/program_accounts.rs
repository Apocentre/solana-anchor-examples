use anchor_lang::prelude::*;

#[account]
pub struct State {
  pub auth_provider: Pubkey,
  pub treasury: Pubkey,
  pub purchase_token: Pubkey,
  pub total_raised: u64,
}

#[account]
#[derive(Default)]
pub struct UserInfo {
  pub total_amount: u64,
}
