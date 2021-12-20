use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod basic_2 {
  use super::*;

  pub fn create(ctx: Context<Create>, authority: Pubkey) -> ProgramResult {
    let counter = &mut ctx.accounts.counter;
    counter.authority = authority;
    counter.count = 0;

    Ok(())
  }

  pub fn increment(ctx: Context<Increment>) -> ProgramResult {
    let counter = &mut ctx.accounts.counter;
    counter.count += 1;

    Ok(())
  }
}

#[derive(Accounts)]
pub struct Create<'info> {
  #[account(init, payer = user, space = 8 + 40)]
  pub counter: Account<'info, Counter>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
  // mut: tells the program to persist all changes to the account.
  // has_one: enforces the constraint that Increment.counter.authority == Increment.authority.key
  #[account(mut, has_one = authority)]
  pub counter: Account<'info, Counter>,
  pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
  pub authority: Pubkey,
  count: u64,
}
