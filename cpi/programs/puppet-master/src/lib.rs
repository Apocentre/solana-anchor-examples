use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use solana_program::{
  instruction::{Instruction},
  program::{invoke_signed},
};
// load the relevant data from the puppet program
use puppet::cpi::accounts::{SetData, SetDataAuth};
use puppet::program::Puppet;
use puppet::{self, State};

declare_id!("7UBgahynaRRc7j5qKm5d4NJCXwMssPjWvRshXu2WPKT9");

#[program]
pub mod puppet_master {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>, bump_seed: u8) -> ProgramResult {
    let puppet_master_pda = &mut ctx.accounts.puppet_master_pda;
    puppet_master_pda.bump_seed = bump_seed;

    Ok(())
  }

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
    let puppet_master_pda = &ctx.accounts.puppet_master_pda;
    let puppet_master_pda_account = puppet_master_pda.to_account_info();
    let seeds: &[&[u8]] = &[
      b"puppet_master_6",
      &[puppet_master_pda.bump_seed]
    ];
    let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
  
    let cpi_accounts = SetDataAuth {
      puppet: ctx.accounts.puppet.to_account_info(),
      authority: puppet_master_pda_account.clone(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
      cpi_program.clone(),
      cpi_accounts,
      signer_seeds
    );

    
    // The following verbose way is what the puppet::cpi::set_data_auth will fo under the hood 
    //
    // let ix_data = (puppet::instruction::SetDataAuth{data}).data();

    // let ix = Instruction {
    //   program_id: *cpi_program.key,
    //   accounts: cpi_ctx.to_account_metas(Some(true)),
    //   data: ix_data,
    // };

    // invoke_signed(
    //   &ix,
    //   &[
    //     ctx.accounts.puppet.to_account_info(),
    //     puppet_master_pda_account,
    //   ],
    //   &signer_seeds
    // )
    puppet::cpi::set_data_auth(cpi_ctx, data)
  }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
  #[account(mut)]
  pub puppet: Account<'info, State>,
  pub puppet_program: Program<'info, Puppet>,
}

#[derive(Accounts)]
#[instruction(bump_seed: u8)]
pub struct Initialize<'info> {
  #[account(
    init,
    space = 8 + 8,
    payer = user,
    seeds = [b"puppet_master_6"],
    bump = bump_seed
  )]
  pub puppet_master_pda: Account<'info, PuppetMasterState>,
  
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PullStringsAuth<'info> {
  #[account(mut)]
  pub puppet: Account<'info, State>,
  pub puppet_program: Program<'info, Puppet>,
  #[account(mut)]
  pub puppet_master_pda: Account<'info, PuppetMasterState>,
}

#[account]
pub struct PuppetMasterState {
  bump_seed: u8,
}
