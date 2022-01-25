use anchor_lang::prelude::*;
use crate::math::{U64};

#[account]
pub struct State {
  pub auth_provider: Pubkey,
  pub total_raised: U64,
}

#[account]
#[derive(Default)]
pub struct UserInfo {
  pub total_amount: U64,
}
