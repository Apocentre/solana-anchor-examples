import anchor from '@project-serum/anchor'
import {expect} from 'chai'
import idl from '../target/idl/basic_1.json'

const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe('basic-1', () => {
  const provider = anchor.Provider.local()
  let program

  before(() => {
    // set the local provider
    anchor.setProvider(provider)

    // address of the deployed program
    const programId = new PublicKey('8VYTPhUNEWkmyKDxRR1EWjN2RufGYc7ZdZU7MnmLe7Po')

    // create the program instance
    program = new anchor.Program(idl, programId)
  })

  const createAccount = async () => {
    const myAccount = Keypair.generate()
    await program.rpc.initialize(new anchor.BN(123), {
      accounts: {
        myAccount: myAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [myAccount]
    })

    return myAccount
  }

  it('should create and init a new account', async () => {
    const myAccount = await createAccount()

    // fetch the newly created account
    const account = await program.account.myAccount.fetch(myAccount.publicKey)
    expect(account.data.toString()).to.be.equal((new anchor.BN(123)).toString())
  })

  it('should update state of an existing account', async () => {
    const myAccount = await createAccount()

    // update state
    await program.rpc.update(new anchor.BN(321), {
      accounts: {
        myAccount: myAccount.publicKey
      }
    })

    const account = await program.account.myAccount.fetch(myAccount.publicKey)
    expect(account.data.toString()).to.be.equal((new anchor.BN(321)).toString())
  })
})
