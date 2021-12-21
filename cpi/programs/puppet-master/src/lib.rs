use anchor_lang::prelude::*;
// load the relevant data from the puppet program
use puppet::cpi::accounts::{SetData, SetDataAuth};
use puppet::program::Puppet;
use puppet::{self, State};

declare_id!("7UBgahynaRRc7j5qKm5d4NJCXwMssPjWvRshXu2WPKT9");

#[program]
pub mod puppet_master {
  use super::*;
  pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> ProgramResult {
    let cpi_program = ctx.accounts.puppet_program.to_account_info();
    let cpi_accounts = SetData {
      puppet: ctx.accounts.puppet.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    puppet::cpi::set_data(cpi_ctx, data)
  }

  pub fn pull_strings_auth(ctx: Context<PullStringsAuth>, data: u64) -> ProgramResult {
    let cpi_program = ctx.accounts.puppet_program.to_account_info();
    let puppet_master_pda = ctx.accounts.puppet_master_pda.to_account_info();

    let seeds: &[&[u8]] = &[b"puppet_master"];
    let signer_seeds = &[&seeds[..]];
    let (pda,_) = Pubkey::find_program_address(seeds, ctx.program_id);

    // log for debugging purposes
    msg!(&format!("PDA -----> {:?}", pda));
    msg!(&format!("ctx.program_id -----> {:?}", ctx.program_id));

    // As all account addresses accessed by an on-chain Solana program must be
    // explicitly passed to the program, it is typical for the PDAs to be
    // derived in off-chain client programs, avoiding the compute cost of
    // generating the address on-chain. The address may or may not then be
    // verified by re-deriving it on-chain, depending on the requirements of
    // the program. This verification may be performed without the overhead of
    // re-searching for the bump key by using the [`create_program_address`]
    // function.
    if pda != *(puppet_master_pda.key) {
      return Err(ErrorCode::WrongPDA.into())
    }

    let cpi_accounts = SetDataAuth {
      puppet: ctx.accounts.puppet.to_account_info(),
      authority: puppet_master_pda,
    };

    let cpi_ctx = CpiContext::new_with_signer(
      cpi_program,
      cpi_accounts,
      signer_seeds
    );
    
    puppet::cpi::set_data_auth(cpi_ctx, data)
  }
}

#[event]
pub struct LogPDA {
  pub pda: Pubkey,
}

#[error]
pub enum ErrorCode {
  #[msg("wrong PDA account")]
  WrongPDA,
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
  #[account(mut)]
  pub puppet: Account<'info, State>,
  pub puppet_program: Program<'info, Puppet>,
}

#[derive(Accounts)]
pub struct PullStringsAuth<'info> {
  #[account(mut)]
  pub puppet: Account<'info, State>,
  pub puppet_program: Program<'info, Puppet>,
  pub puppet_master_pda: AccountInfo<'info>
}
