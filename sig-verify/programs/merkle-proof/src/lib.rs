use anchor_lang::prelude::*;

declare_id!("3th4VMLzB3Qz7wcnZUoL57fbXqBG1QvVMAHDm6PBURmH");

#[program]
pub mod merkle_proof {
  use super::*;
  pub fn initialize(ctx: Context<Initialize>, merkle_root: [u8; 32]) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    state.merkle_root = merkle_root;

    Ok(())
  }

  pub fn contribute(
    ctx: Context<Contribute>,
    _proof: Vec<[u8; 32]>,
    amount: u64,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    let raw_message = Entry {
      sender: ctx.accounts.sender.unsigned_key(),
      amount: amount,
    };

    let mut message: Vec<u8> = Vec::new();
    raw_message.serialize(&mut message).unwrap();

    msg!("encode leaf {:?}", message);

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

#[account]
pub struct State {
  merkle_root: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Entry<'info> {
  sender: &'info Pubkey,
  amount: u64,
}


#[error]
pub enum ErrorCode {
  #[msg("merkle proof failed")]
  InvalidSig,
}
