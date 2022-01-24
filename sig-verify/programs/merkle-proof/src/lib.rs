pub mod merkle_tree;

use anchor_lang::prelude::*;
use merkle_tree::hasher::{BoshHasher};
use rs_merkle::MerkleProof;

declare_id!("3th4VMLzB3Qz7wcnZUoL57fbXqBG1QvVMAHDm6PBURmH");

#[program]
pub mod merkle_proof {
  use super::*;
  pub fn initialize(
    ctx: Context<Initialize>,
    merkle_root: [u8; 32],
    merkle_root_count: usize,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    state.merkle_root = merkle_root;
    state.merkle_root_count = merkle_root_count;

    Ok(())
  }

  pub fn contribute(
    ctx: Context<Contribute>,
    proof: Vec<[u8; 32]>,
    proof_indices: Vec<usize>,
    _amount: u64,
  ) -> ProgramResult {
    let state = &mut ctx.accounts.state;
    let raw_message = Entry {
      sender: ctx.accounts.sender.unsigned_key()
    };
    let mut message: Vec<u8> = Vec::new();
    raw_message.serialize(&mut message).unwrap();

    msg!("proof_indices {:#?}", proof_indices);
    msg!("proof {:#?}", proof);
    msg!("encoded leaf {:?}", message);

    let merkle_proof = MerkleProof::<BoshHasher>::new(proof.clone());
    merkle_proof.verify(
      state.merkle_root,
      &proof_indices,
      &proof,
      state.merkle_root_count,
    );

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
  merkle_root_count: usize,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Entry<'info> {
  sender: &'info Pubkey,
}

#[error]
pub enum ErrorCode {
  #[msg("merkle proof failed")]
  InvalidSig,
}
