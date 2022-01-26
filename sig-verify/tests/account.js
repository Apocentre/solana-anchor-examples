import anchor from '@project-serum/anchor'

const {
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  Transaction
} = anchor.web3

export const createAccount = async provider => {
  const newAccount = Keypair.generate()
  const createAccountIx = SystemProgram.createAccount({
    programId: SystemProgram.programId,
    fromPubkey: provider.wallet.publicKey,
    newAccountPubkey: newAccount.publicKey
  })

  const tx = new Transaction()
  tx.add(createAccountIx)
  const {blockhash} = await provider.connection.getRecentBlockhash()
  tx.recentBlockhash = blockhash
  tx.feePayer = provider.wallet.publicKey
  
  await provider.wallet.signTransaction(tx)
  
  // this will make the provider wallet sign the tx, as well as, a list of
  // array of signers (in this case the new account that is created should also sign the tx)
  await provider.send(tx, [newAccount])
  await airdrop(provider, newAccount)
  
  return newAccount
}

export const airdrop = async (provider, account) => {
  const airdropSignature = await provider.connection.requestAirdrop(
    account.publicKey,
    LAMPORTS_PER_SOL,
  )

  await provider.connection.confirmTransaction(airdropSignature)
}
