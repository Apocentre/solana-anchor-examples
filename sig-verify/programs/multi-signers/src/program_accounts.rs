use anchor_lang::prelude::*;

#[account]
pub struct State {
  pub auth_provider: Pubkey,
}
