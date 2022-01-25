pub mod program_accounts;
pub mod program_errors;
pub mod program_access_controls;

use anchor_lang::prelude::*;


declare_id!("7m5hgk2TdJUJ4RX3paZg3EsPTuagphT5XT4MyZq4qy6J");

use program_accounts::{State};
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
    amount: u64,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;


    // TODO: we know the user is authenticated thus we can continue with the business logic
    msg!("amount {:?}", amount);

    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + 32
  )]
  pub state: Account<'info, State>,

  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,
  #[account()]
  pub sender: Signer<'info>,
  #[account()]
  pub auth_provider: Signer<'info>,
}
