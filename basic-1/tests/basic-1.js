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

  it('should create and init a new account', async () => {
    const myAccount = Keypair.generate()
    await program.rpc.initialize(new anchor.BN(123), {
      accounts: {
        myAccount: myAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [myAccount]
    })

    // fetch the newly created account
    const account = await program.account.myAccount.fetch(myAccount.publicKey)
    expect(account.data.toString()).to.be.equal((new anchor.BN(123)).toString())
  })
})
