import {Token, TOKEN_PROGRAM_ID} from "@solana/spl-token"

export const createMintAccount = async (
  connection,
  feePayer,
  mintAuthority,
  decimals=9,
  freezeAuthority=null,
) => await Token.createMint(
  connection,
  feePayer,
  mintAuthority,
  freezeAuthority,
  decimals,
  TOKEN_PROGRAM_ID,
)

export const createTokenAccount = async (token, owner) => await token.getOrCreateAssociatedAccountInfo(owner)

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

export const getTokenProgramId = () => TOKEN_PROGRAM_ID
