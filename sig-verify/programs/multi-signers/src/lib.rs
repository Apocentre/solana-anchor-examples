pub mod program_accounts;
pub mod safe_math;
pub mod program_errors;
pub mod program_access_controls;

use anchor_lang::prelude::*;
use std::mem::size_of;
use std::convert::Into;
use safe_math::{SafeMath};

declare_id!("7m5hgk2TdJUJ4RX3paZg3EsPTuagphT5XT4MyZq4qy6J");

use program_accounts::{State, UserInfo};
use program_access_controls::{authenticate};

#[program]
pub mod multi_signers {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>, auth_provider: Pubkey) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    state.auth_provider = auth_provider;

    Ok(())
  }

  #[access_control(authenticate(&ctx.accounts.auth_provider, &ctx.accounts.state))]
  pub fn contribute(
    ctx: Context<Contribute>,
    _bump_seed: u8, // NOTE: make sure this is the first param user injects; otherwise it doesn't work
    amount: u64,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    let user_state = &mut ctx.accounts.user_state;

    state.total_raised = state.total_raised.safe_add(amount)?;
    user_state.total_amount = user_state.total_amount.safe_add(amount)?;

    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + size_of::<State>(),
  )]
  pub state: Account<'info, State>,

  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump_seed: u8)]
pub struct Contribute<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,
  #[account(
    init,
    payer = user,
    space = 8 + size_of::<UserInfo>(),
    seeds = [b"multi_signers", user.key().as_ref()],
    bump = bump_seed,
  )]
  pub user_state: Account<'info, UserInfo>,
  #[account(mut)]
  pub user: Signer<'info>,
  #[account()]
  pub auth_provider: Signer<'info>,
  pub system_program: Program<'info, System>,
}
