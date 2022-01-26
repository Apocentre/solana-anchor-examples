import { token } from "@project-serum/anchor/dist/cjs/utils"
import {Token, TOKEN_PROGRAM_ID} from "@solana/spl-token"

export const createMintAccount = async (
  connection,
  feePayer,
  mintAuthority,
  freezeAuthority,
  decimals=9
) => await Token.createMint(
  connection,
  feePayer,
  mintAuthority,
  freezeAuthority,
  decimals,
  TOKEN_PROGRAM_ID,
)

export const createTokenAccount = async (token, account) => await token.getOrCreateAssociatedAccountInfo(account)

export const mintTo = async (
  token,
  dest,
  authority,
  amount,
  multiSigners=[]
) => await token.mintTo(
  dest,
  authority,
  multiSigners,
  amount,
)

export const transfer = async (
  sourceTokenAccount,
  dest,
  sourceTokenAccountOwner,
  amount,
  multiSigners=[]
) => await token.transfer(
  sourceTokenAccount,
  dest,
  sourceTokenAccountOwner,
  multiSigners,
  amount
)

export const approve = async (
  sourceTokenAccount,
  delegate,
  sourceTokenAccountOwner,
  amount,
  multiSigners=[]
) => await token.approve(
  sourceTokenAccount,
  delegate,
  sourceTokenAccountOwner,
  multiSigners,
  amount
)
