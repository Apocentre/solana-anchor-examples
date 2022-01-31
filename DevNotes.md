https://discord.com/channels/889577356681945098/889577399308656662/889704874479079485
https://discord.com/channels/889577356681945098/889577399308656662/918472062182494248
https://discord.com/channels/889577356681945098/889584618372734977/917778169241665557

Errors
===

> find value `rent` in this scope

This usually happens when we want to create a new TokenAccount. To fix this we need to include the `Sysvar<'info, Rent>` in the accounts.

For example,

```
#[derive(Accounts)]
#[instruction(bump_seed: u8)]
pub struct Initialize<'info> {
  #[account(
    init_if_needed,
    payer = user,
    space = 8 + size_of::<State>(),
    seeds = [b"iho_program"],
    bump = bump_seed,
  )]
  pub state: Account<'info, State>,
  #[account(
    init_if_needed,
    payer = user,
    space = 8 + size_of::<TokenAccount>(),
    token::mint = offering_token,
    token::authority = state,
  )]
  pub vault_token_account: Account<'info, TokenAccount>,
  #[account()]
  pub offering_token: Account<'info, Mint>,
  
  #[account(mut)]
  pub user: Signer<'info>,
  token_program: Program<'info, Token>,
  system_program: Program<'info, System>,

  // this is needed to create the token account
  rent: Sysvar<'info, Rent>,
}
```
