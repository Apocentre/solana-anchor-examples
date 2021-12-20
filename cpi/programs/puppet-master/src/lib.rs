use anchor_lang::prelude::*;
// load the relevant data from the puppet program
use puppet::cpi::accounts::SetData;
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
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
  #[account(mut)]
  pub puppet: Account<'info, State>,
  pub puppet_program: Program<'info, Puppet>,
}
