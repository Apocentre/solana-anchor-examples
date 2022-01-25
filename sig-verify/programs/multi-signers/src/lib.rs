use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod multi_signers {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>, auth_provider: Pubkey) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    state.auth_provider = auth_provider;

    Ok(())
  }

  pub fn contribute(
    ctx: Context<Contribute>,
    amount: u64,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;

    // tx should be signed by both the sender and the auth provider
    if ctx.accounts.auth_provider.unsigned_key() != &state.auth_provider {
      return Err(ErrorCode::Unauthorized.into())
    }

    // TODO: we know the user is authenticated thus we can continue with the business logic
    msg!("amount {:?}", amount);

    Ok(())
  }
}

#[account]
pub struct State {
  auth_provider: Pubkey,
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

#[error]
pub enum ErrorCode {
  #[msg("unauthorized")]
  Unauthorized,
}
