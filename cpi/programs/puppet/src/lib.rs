use anchor_lang::prelude::*;

declare_id!("7upcW754B7BSCJMZj5Vts3VFtbRYPrEFAtkDzNd2reuP");

#[program]
pub mod puppet {
  use super::*;
  
  pub fn initialize(ctx: Context<Initialize>, puppet_master: Pubkey) -> ProgramResult {
    let puppet = &mut ctx.accounts.puppet;
    puppet.data = 0;
    puppet.authority = puppet_master;

    Ok(())
  }

  pub fn set_data(ctx: Context<SetData>, data: u64) -> ProgramResult {
    let puppet = &mut ctx.accounts.puppet;
    puppet.data = data;

    Ok(())
  }

  pub fn set_data_auth(ctx: Context<SetDataAuth>, data: u64) -> ProgramResult {
    let puppet = &mut ctx.accounts.puppet;
    
    // it can inly be called via CPI from the pupper master
    if puppet.authority != ctx.accounts.authority.key() {
      return Err(ErrorCode::Unauthorized.into())
    }
    
    puppet.data = data;

    Ok(())
  }
}

#[error]
pub enum ErrorCode {
  #[msg("wrong PDA account")]
  Unauthorized,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 8 + 40)]
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

#[derive(Accounts)]
pub struct SetDataAuth<'info> {
  #[account(mut)]
  pub puppet: Account<'info, State>,
  pub authority: AccountInfo<'info>,
}

#[account]
pub struct State {
  pub authority: Pubkey,
  data: u64,
}
