use anchor_lang::prelude::*;

declare_id!("7upcW754B7BSCJMZj5Vts3VFtbRYPrEFAtkDzNd2reuP");

#[program]
pub mod puppet {
  use super::*;
  
  pub fn initialize(ctx: Context<Initialize>, puppet_master_pda: Pubkey) -> ProgramResult {
    let puppet = &mut ctx.accounts.puppet;
    puppet.data = 0;
    puppet.puppet_master_pda = puppet_master_pda;

    Ok(())
  }

  pub fn set_data(ctx: Context<SetData>, data: u64) -> ProgramResult {
    let puppet = &mut ctx.accounts.puppet;
    puppet.data = data;

    Ok(())
  }

  pub fn set_data_auth(ctx: Context<SetDataAuth>, data: u64) -> ProgramResult {
    let puppet = &mut ctx.accounts.puppet;
    let authority = &ctx.accounts.authority;

    if !authority.is_signer && puppet.puppet_master_pda != authority.key() {
      return Err(ErrorCode::NotPuppetMaster.into())
    }

    puppet.data = data;

    Ok(())
  }
}

#[error]
pub enum ErrorCode {
  #[msg("only puppet master")]
  NotPuppetMaster,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + 40
  )]
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
  #[account(mut)]
  pub authority: Signer<'info>,
}

#[account]
pub struct State {
  puppet_master_pda: Pubkey,
  data: u64,
}
