import anchor from '@project-serum/anchor';
import {expect} from 'chai'
import idl from '../target/idl/basic_2.json'

const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe('basic-2', () => {
  const provider = anchor.Provider.local()
  let program

  before(() => {
    anchor.setProvider(provider)

    const programId = new PublicKey('2NKe7BxFyyP5SL84e4n7sjQfGEGVyK9De1eGBr3uLs8N')
    program = new anchor.Program(idl, programId)
  })

  const create = async () => {
    const counterAccount = Keypair.generate()

    await program.rpc.create(provider.wallet.publicKey, {
      accounts: {
        counter: counterAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [counterAccount, provider.wallet.payer]
    })

    return counterAccount
  }

  it('create a new counter account', async () => {
    const counterAccount = await create()

    const account = await program.account.counter.fetch(counterAccount.publicKey)
    expect(account.authority.equals(provider.wallet.publicKey)).to.be.true
    expect(account.count.toNumber()).to.be.equal(0)
  })

  it('should allow the authority to increment the value', async () => {
    const counterAccount = await create()

    await program.rpc.increment({
      accounts: {
        counter: counterAccount.publicKey,
        authority: provider.wallet.publicKey,
      },
      // this is optional by default web3 will use the provider's wallet to sign the transaction
      signers: [provider.wallet.payer]
    })

    const account = await program.account.counter.fetch(counterAccount.publicKey)
    expect(account.authority.equals(provider.wallet.publicKey)).to.be.true
    expect(account.count.toNumber()).to.be.equal(1)
  })

  it('should fail if non-authority account calls increment', async () => {
    const counterAccount = await create()
    const Chunk = Keypair.generate()

    // Chunk tries to increment the counter which he is not the authority 
    try {
      await program.rpc.increment({
        accounts: {
          counter: counterAccount.publicKey,
          authority: Chunk.publicKey,
        },
        signers: [Chunk]
      })
    }
    catch(error) {
      expect(error.message).to.equal('2001: A has_one constraint was violated')
    }
  })
})
