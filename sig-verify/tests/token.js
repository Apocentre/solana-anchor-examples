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
