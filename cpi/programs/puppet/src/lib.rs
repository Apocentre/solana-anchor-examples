use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod puppet {
  use super::*;
  
  pub fn initialize(_: Context<Initialize>) -> ProgramResult {
    Ok(())
  }

  pub fn set_data(ctx: Context<SetData>, data: u64) -> ProgramResult {
    let puppet = &mut ctx.accounts.puppet;
    puppet.data = data;
    
    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 8 + 8)]
  pub puppet: Account<'info, State>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
  #[account(mut)]
  pub puppet: Account<'info, State>,
}

#[account]
pub struct State {
  pub data: u64,
}
