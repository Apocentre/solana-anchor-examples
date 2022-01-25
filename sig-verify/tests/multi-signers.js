import anchor from '@project-serum/anchor'
import {use, expect} from 'chai'
import chaiAsPromise from 'chai-as-promised'

use(chaiAsPromise)
const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe.only('multi-signers', () => {
  const provider = anchor.Provider.local()
  anchor.setProvider(provider)

  const program = anchor.workspace.MultiSigners
  const authProvider = Keypair.generate()
  const stateAccount = Keypair.generate()

  const createTx = async (
    feePayer,
    amount,
    blockhash
  ) => {
    const tx = await program.transaction.contribute(amount, {
      accounts: {
        state: stateAccount.publicKey,
        sender: provider.wallet.publicKey,
        authProvider: authProvider.publicKey,
      }
    })
    tx.recentBlockhash = blockhash
    tx.feePayer = feePayer

    return tx
  }

  // Note we use different way to sign for the local provider since that might include some
  // browser extension that will be signing and thus we don't have direct access to the private key
  const providerSign = async tx => {
    await provider.wallet.signTransaction(tx)

    return tx.signatures
      .find(s => s.publicKey.equals(provider.wallet.publicKey))
      .signature
      .toString('hex')
  }

  const partiallySign = async (tx, signer) => {
    tx.sign(signer)
    
    return tx.signatures
      .find(s => s.publicKey.equals(signer.publicKey))
      .signature
      .toString('hex')
  }

  const initialize = async () => {
    await program.rpc.initialize(authProvider.publicKey, {
      accounts: {
        state: stateAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [stateAccount, provider.wallet.payer]
    })
  }

  it('should initialize', async () => {
    await initialize()

    const account = await program.account.state.fetch(stateAccount.publicKey)
    expect(account.authProvider.toString()).to.equal(authProvider.publicKey.toString())
  });

  it('should fail if tx is not signed by both the sender and the auth provider', async () => {
    const amount = new anchor.BN(5000)
    const {blockhash} = await provider.connection.getRecentBlockhash()
    // create the transaction that will be signed by both signers
    const tx = await createTx(
      provider.wallet.publicKey,
      amount,
      blockhash
    )
    
    const authProviderSig = await partiallySign(tx, authProvider)
    const senderSig = await providerSign(tx)

    // compile all signatures to complete the tx
    tx.addSignature(authProvider.publicKey, Buffer.from(authProviderSig, 'hex'))
    tx.addSignature(provider.wallet.publicKey, Buffer.from(senderSig, 'hex'))

    // simulate sending the serialized and partially signed tx to the final signer
    // that will transmit the tx to the network
    const result = await provider.connection.sendRawTransaction(tx.serialize())
    
    console.log('Result tx ', result)
  })
});
