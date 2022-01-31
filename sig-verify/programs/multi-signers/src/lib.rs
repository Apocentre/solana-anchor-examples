pub mod program_accounts;
pub mod safe_math;
pub mod program_errors;
pub mod program_access_controls;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use std::mem::size_of;
use std::convert::Into;
use anchor_safe_math::{SafeMath};
use program_errors::{ErrorCode};

declare_id!("7m5hgk2TdJUJ4RX3paZg3EsPTuagphT5XT4MyZq4qy6J");

use program_accounts::{State, UserInfo};
use program_access_controls::{authenticate};

#[program]
pub mod multi_signers {
  use super::*;

  pub fn initialize(
    ctx: Context<Initialize>,
    auth_provider: Pubkey,
    treasury: Pubkey,
    purchase_token: Pubkey
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    state.auth_provider = auth_provider;
    state.treasury = treasury;
    state.purchase_token = purchase_token;

    Ok(())
  }

  // NOTE: This has the same effect as the constraint we added to user_token_account
  // #[access_control(check_token(&ctx.accounts.user_token_account.mint, &ctx.accounts.state.purchase_token))]
  #[access_control(authenticate(&ctx.accounts.auth_provider, &ctx.accounts.state))]
  pub fn contribute(
    ctx: Context<Contribute>,
    _bump_seed: u8, // NOTE: make sure this is the first param user injects; otherwise it doesn't work
    amount: u64,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    let user_state = &mut ctx.accounts.user_state;

    // transfer tokens from user's token Account to the treasury account
    let cpi_accounts = Transfer {
      from: ctx.accounts.user_token_account.to_account_info(),
      to: ctx.accounts.treasury_account.to_account_info(),
      authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    state.total_raised = state.total_raised.safe_add(amount)?;
    user_state.total_amount = user_state.total_amount.safe_add(amount)?;

    Ok(())
  }
}

#[derive(Accounts)]
#[instruction(bump_seed: u8)]
pub struct Initialize<'info> {
  // This is the global state for an instance of this program
  #[account(
    init,
    payer = user,
    space = 8 + size_of::<State>(),
  )]
  pub state: Account<'info, State>,
  #[account(mut)]
  pub user: Signer<'info>,

  // These two are needed to create of the above accounts
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump_seed: u8)]
pub struct Contribute<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,
  // this is the user state PDA which stores the state for the given user
  // it uses the user public key and thus it's unique for each user
  #[account(
    init_if_needed,
    payer = user,
    space = 8 + size_of::<UserInfo>(),
    seeds = [b"multi_signers", user.key().as_ref()],
    bump = bump_seed,
  )]
  pub user_state: Account<'info, UserInfo>,
  #[account(mut)]
  pub user: Signer<'info>,
  // The token account owned by the user that call contribute
  // the constraint suggest both this token account matched the 
  // purchase token used for this sale
  #[account(
    mut,
    constraint = user_token_account.mint == state.purchase_token @ ErrorCode::UnsupportedToken,
    constraint = user_token_account.owner == user.key() @ ErrorCode::WrongTokenAccountOwner
  )]
  pub user_token_account: Account<'info, TokenAccount>,
  #[account(
    mut,
    // The token account owned by the treasury that stores all the raised funds
    // The constraint suggests that both this token account and the treasury point to the
    // same mint account i.e. Token
    constraint = user_token_account.mint == treasury_account.mint @ ErrorCode::UnsupportedToken,
  )]
  pub treasury_account: Account<'info, TokenAccount>,
  
  // The account that should co-sign the contribute operation to provide access to the user
  #[account()]
  pub auth_provider: Signer<'info>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}
