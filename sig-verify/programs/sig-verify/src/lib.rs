use anchor_lang::prelude::*;
use borsh::BorshSerialize;
use std::convert::TryFrom;

declare_id!("GTRqnYcgFdKRWrBA8eujeVN7kWfNgdQX1XH77SPfqoya");

#[program]
pub mod sig_verify {
  use super::*;
  pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    state.authority = authority;

    Ok(())
  }

  pub fn contribute(
    ctx: Context<Contribute>,
    nonce: u64,
    sig: [u8; 64],
    amount: u64,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    let raw_message = ContributeMsg {
      program: ctx.program_id,
      nonce: nonce,
      sender: ctx.accounts.sender.unsigned_key(),
      amount: amount
    };

    let mut message: Vec<u8> = Vec::new();
    raw_message.serialize(&mut message).unwrap();
    
    let key = salty::signature::PublicKey::try_from(&state.authority.to_bytes()).unwrap();
    let sig = salty::signature::Signature::from(&sig);

    msg!("message {:?}", message);
    msg!("sig {:?}", sig);

    if let Err(_) = key.verify(&message, &sig) {
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
  nonce: u64,
  sender: &'info Pubkey,
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
