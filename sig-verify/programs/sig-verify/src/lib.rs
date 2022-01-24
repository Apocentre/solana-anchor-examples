use anchor_lang::prelude::*;
use borsh::BorshSerialize;
use sodalite::sign_attached_open;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sig_verify {
  use super::*;
  pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    state.authority = authority;

    Ok(())
  }

  pub fn contribute(ctx: Context<Contribute>, data: ContributeData) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    let raw_message = ContributeMsg {
      program: ctx.program_id,
      nonce: data.nonce,
      sender: ctx.accounts.sender.unsigned_key(),
      amount: data.amount
    };

    let mut message: Vec<u8> = Vec::new();
    raw_message.serialize(&mut message).unwrap();

    if let Err(_) = sign_attached_open(
      &mut message,
      &data.sig,
      &state.authority.to_bytes()
    ) {
      return Err(ErrorCode::InvalidSig.into())
    }

    Ok(())
  }
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
  #[account(mut)]
  pub sender: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ContributeMsg<'info> {
  program: &'info Pubkey,
  nonce: [u8; 32],
  sender: &'info Pubkey,
  amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ContributeData {
  nonce: [u8; 32],
  sig: [u8; 64],
  amount: u64,
}

#[account]
pub struct State {
  authority: Pubkey,
}

#[error]
pub enum ErrorCode {
  #[msg("invalid sig")]
  InvalidSig,
}
