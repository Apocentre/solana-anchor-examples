import anchor from '@project-serum/anchor'
import {use, expect} from 'chai'
import chaiAsPromise from 'chai-as-promised'

use(chaiAsPromise)
const {SystemProgram, PublicKey, Keypair, utils} = anchor.web3
const utf8 = anchor.utils.bytes.utf8

describe.only('multi-signers', () => {
  const provider = anchor.Provider.local()
  anchor.setProvider(provider)

  const program = anchor.workspace.MultiSigners
  const authProvider = Keypair.generate()
  const stateAccount = Keypair.generate()
  let pda
  let bump_seed

  before(async () => {
    [pda, bump_seed] = await createAccountInfoPDA(user)
  })
  const createAccountInfoPDA = async user => {
    return await PublicKey.findProgramAddress(
      [utf8.encode('multi_signers'), user.toBuffer()],
      program.programId
    )
  }

  const createTx = async (
    user,
    amount,
    blockhash
  ) => {
    const tx = await program.transaction.contribute(bump_seed, amount, {
      accounts: {
        state: stateAccount.publicKey,
        user,
        userState: pda,
        authProvider: authProvider.publicKey,
        systemProgram: SystemProgram.programId
      }
    })
    tx.recentBlockhash = blockhash
    tx.feePayer = user

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
  })

  it('should fail if tx is not signed by both the sender and the auth provider', async () => {
    const amount = new anchor.BN(5000)
    const {blockhash} = await provider.connection.getRecentBlockhash()
    // create the transaction that will be signed by both signers
    const tx = await createTx(
      provider.wallet.publicKey,
      amount,
      blockhash
    )

    const dodgyAccount = Keypair.generate()
    const authProviderSig = await partiallySign(tx, dodgyAccount)
    const senderSig = await providerSign(tx)

    // A dodgy account tries to sign the tx and this authenticate the user
    tx.addSignature(dodgyAccount.publicKey, Buffer.from(authProviderSig, 'hex'))
    tx.addSignature(provider.wallet.publicKey, Buffer.from(senderSig, 'hex'))

    await expect(provider.connection.sendRawTransaction(tx.serialize())).to.be.rejectedWith('unauthorized')
  })

  it('should update the global and user state', async () => {
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
    
    const state = await program.account.state.fetch(stateAccount.publicKey)
    const userState = await program.account.state.fetch(pda)
    
    expect(state.totalRaised).to.equal(new anchor.BN(5000))
    expect(userState.totalAmount).to.equal(new anchor.BN(5000))
  })
})
