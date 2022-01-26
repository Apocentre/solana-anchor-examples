import anchor from '@project-serum/anchor'
import {use, expect} from 'chai'
import chaiAsPromise from 'chai-as-promised'
import {createAccount} from './account.js'
import {
  getTokenProgramId,
  createMintAccount,
  createTokenAccount,
  mintTo,
  getAccountInfo
} from './token.js'

use(chaiAsPromise)
const {SystemProgram, PublicKey, Keypair} = anchor.web3
const utf8 = anchor.utils.bytes.utf8


describe.only('multi-signers', () => {
  const provider = anchor.Provider.local()
  anchor.setProvider(provider)

  const program = anchor.workspace.MultiSigners
  const authProvider = Keypair.generate()

  let stateAccount
  let owner
  let token
  let treasuryAccount
  let pda
  let bump_seed

  const createAccountInfoPDA = async user => {
    return await PublicKey.findProgramAddress(
      [utf8.encode('multi_signers'), user.toBuffer()],
      program.programId
    )
  }

  const createTx = async (
    user,
    authProvider,
    userTokenAccount,
    treasuryAccount,
    amount,
    blockhash
  ) => {
    [pda, bump_seed] = await createAccountInfoPDA(user)
    const tx = await program.transaction.contribute(bump_seed, amount, {
      accounts: {
        state: stateAccount.publicKey,
        user,
        userState: pda,
        authProvider,
        userTokenAccount,
        treasuryAccount,
        tokenProgram: getTokenProgramId(),
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
    await program.rpc.initialize(
      authProvider.publicKey,
      treasuryAccount.address, 
      token.publicKey, 
      {
        accounts: {
          state: stateAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId
        },
        signers: [stateAccount, provider.wallet.payer]
      }
    )
  }

  beforeEach(async () => {
    stateAccount = Keypair.generate()
    owner = await createAccount(provider)
    token = await createMintAccount(
      provider.connection,
      owner,
      owner.publicKey
    )
    treasuryAccount = await createTokenAccount(token, owner.publicKey)
  })

  it('should initialize', async () => {
    await initialize()

    const account = await program.account.state.fetch(stateAccount.publicKey)
    expect(account.authProvider.toString()).to.equal(authProvider.publicKey.toString())
  })

  it('should fail if tx is not signed by both the sender and the auth provider', async () => {
    await initialize()

    const amount = new anchor.BN(5000)
    const dodgyAccount = await createAccount(provider)
    const userTokenAccount = await createTokenAccount(token, provider.wallet.publicKey)
    const {blockhash} = await provider.connection.getRecentBlockhash()
    const tx = await createTx(
      provider.wallet.publicKey,
      dodgyAccount.publicKey,
      userTokenAccount.address,
      treasuryAccount.address,
      amount,
      blockhash
    )

    const authProviderSig = await partiallySign(tx, dodgyAccount)
    const senderSig = await providerSign(tx)

    // A dodgy account tries to sign the tx and not the authenticated the user
    tx.addSignature(dodgyAccount.publicKey, Buffer.from(authProviderSig, 'hex'))
    tx.addSignature(provider.wallet.publicKey, Buffer.from(senderSig, 'hex'))

    // NOTE custom error messages seem to be broken in the latest anchor version
    // await expect(provider.connection.sendRawTransaction(tx.serialize())).to.be.rejectedWith('unauthorized')
    await expect(provider.connection.sendRawTransaction(tx.serialize()))
      .to.be.rejectedWith('Error processing Instruction 0: custom program error: 0x1770')
  })

  it('should update the global and user state', async () => {
    await initialize()

    const alice = await createAccount(provider)
    const userTokenAccount = await createTokenAccount(token, alice.publicKey)
    const amount = new anchor.BN(5000)

    await mintTo(
      token,
      userTokenAccount.address,
      owner.publicKey,
      5000
    )
    
    const {blockhash} = await provider.connection.getRecentBlockhash()
    // create the transaction that will be signed by both signers
    const tx = await createTx(
      alice.publicKey,
      authProvider.publicKey,
      userTokenAccount.address,
      treasuryAccount.address,
      amount,
      blockhash
    )
    const authProviderSig = await partiallySign(tx, authProvider)
    const senderSig = await partiallySign(tx, alice)

    // compile all signatures to complete the tx
    tx.addSignature(authProvider.publicKey, Buffer.from(authProviderSig, 'hex'))
    tx.addSignature(alice.publicKey, Buffer.from(senderSig, 'hex'))

    // simulate sending the serialized and partially signed tx to the final signer
    // that will transmit the tx to the network
    await provider.connection.sendRawTransaction(tx.serialize())
    
    const [pda] = await createAccountInfoPDA(alice.publicKey)
    await new Promise(resolve => setTimeout(resolve, 1000))
    const state = await program.account.state.fetch(stateAccount.publicKey)
    const userState = await program.account.userInfo.fetch(pda)

    expect(state.totalRaised.toNumber()).to.equal(5000)
    expect(userState.totalAmount.toNumber()).to.equal(5000)

    // check that treasury has received the tokens.
    // Note we need to load the account info again from the network
    const treasuryAccountInfo = await getAccountInfo(token, treasuryAccount.address)
    expect(treasuryAccountInfo.amount.toNumber()).to.equal(5000)
  })
})
