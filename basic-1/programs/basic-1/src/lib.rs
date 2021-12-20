use anchor_lang::prelude::*;

declare_id!("8VYTPhUNEWkmyKDxRR1EWjN2RufGYc7ZdZU7MnmLe7Po");

#[program]
pub mod basic_1 {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>, data: u64) -> ProgramResult {
    let my_account = &mut ctx.accounts.my_account;
    my_account.data = data;

    Ok(())
  }

  pub fn update(ctx: Context<Update>, data: u64) -> ProgramResult {
    let my_account = &mut ctx.accounts.my_account;
    my_account.data = data;

    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 8 + 8)]
  pub my_account: Account<'info, MyAccount>,

  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

// derive macro implementing the Accounts trait (opens new window), allowing a 
// struct to transform from the untrusted &[AccountInfo] slice given to a Solana
//program into a validated struct of deserialized account types.
#[derive(Accounts)]
pub struct Update<'info> {
  // attribute macro implementing AccountSerialize (opens new window)and AccountDeserialize (opens new window),
  // automatically prepending a unique 8 byte discriminator to the account array.
  // The discriminator is defined by the first 8 bytes of the Sha256 hash of the account's 
  // Rust identifier--i.e., the struct type name--and ensures no account can be substituted for another.
  #[account(mut)]
  // Account is a wrapper type for a deserialized account implementing AccountDeserialize.
  // Using this type within an Accounts struct will ensure the account is owned by the address 
  // defined by declare_id! where the inner account was defined.
  pub my_account: Account<'info, MyAccount>
}

#[account]
pub struct MyAccount {
  pub data: u64
}
