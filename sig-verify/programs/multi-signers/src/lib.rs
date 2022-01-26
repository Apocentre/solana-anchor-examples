pub mod program_accounts;
pub mod safe_math;
pub mod program_errors;
pub mod program_access_controls;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};
use std::mem::size_of;
use std::convert::Into;
use safe_math::{SafeMath};
use program_errors::{ErrorCode};

declare_id!("7m5hgk2TdJUJ4RX3paZg3EsPTuagphT5XT4MyZq4qy6J");

use program_accounts::{State, UserInfo};
use program_access_controls::{authenticate, check_token};

#[program]
pub mod multi_signers {
  use super::*;

  pub fn initialize(
    ctx: Context<Initialize>,
    _bump_seed: u8, // NOTE: make sure this is the first param user injects; otherwise it doesn't work
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

  #[access_control(authenticate(&ctx.accounts.auth_provider, &ctx.accounts.state))]
  #[access_control(check_token(&ctx.accounts.user_token_account.mint, &ctx.accounts.state.purchase_token))]
  pub fn contribute(
    ctx: Context<Contribute>,
    _bump_seed: u8, // NOTE: make sure this is the first param user injects; otherwise it doesn't work
    amount: u64,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    let user_state = &mut ctx.accounts.user_state;

    // transfer tokens from user's token Account to the treasury account
    let user_token_account = &ctx.accounts.user_token_account;
    let cpi_accounts = Transfer {
      from: ctx.accounts.user.to_account_info(),
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
  #[account(
    init,
    payer = user,
    space = 8 + size_of::<State>(),
  )]
  pub state: Account<'info, State>,
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
    init,
    payer = user,
    space = 8 + size_of::<TokenAccount>(),
    token::mint = mint,
    token::authority = user,
  )]
  pub treasury_account: Account<'info, TokenAccount>,
  #[account(address = state.purchase_token)]
  pub mint: Account<'info, Mint>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump_seed: u8)]
pub struct Contribute<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,

  // this is the user state PDA which stores the state for the given user
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
  #[account(
    mut,
    constraint = user_token_account.mint == state.purchase_token @ ErrorCode::UnsupportedToken
  )]
  pub user_token_account: Account<'info, TokenAccount>,
  #[account(
    mut,
    // this is an alternative to using access_control
    constraint = user_token_account.mint == treasury_account.mint @ ErrorCode::UnsupportedToken
  )]
  pub treasury_account: Account<'info, TokenAccount>,
  #[account()]
  pub auth_provider: Signer<'info>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}
