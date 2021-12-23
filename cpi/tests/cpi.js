import anchor from '@project-serum/anchor'
import {expect} from 'chai'
import puppetIDL from '../target/idl/puppet.json'
import puppetMasterIDL from '../target/idl/puppet_master.json'

const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe('cpi', () => {
  const provider = anchor.Provider.local()
  let puppetProgram
  let puppetMasterProgram
  let puppetProgramId
  let puppetMasterProgramId

  const createPuppetMasterPDA = async () => {
    return await PublicKey.findProgramAddress([Buffer.from("puppet_master_6")], puppetMasterProgramId)
  }

  const createPuppetAccount = async () => {
    const puppetAccount = Keypair.generate()
    
    await puppetProgram.rpc.initialize((await createPuppetMasterPDA())[0], {
      accounts: {
        puppet: puppetAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [puppetAccount, provider.wallet.payer]
    })
    
    return puppetAccount
  }

  const initializePda = async () => {
    const [pda, bump_seed] = await createPuppetMasterPDA()
    await puppetMasterProgram.rpc.initialize(bump_seed, {
      accounts: {
        puppetMasterPda: pda,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [provider.wallet.payer]
    })

    return [pda, bump_seed]
  }

  before(() => {
    anchor.setProvider(provider)

    puppetProgramId = new PublicKey('7upcW754B7BSCJMZj5Vts3VFtbRYPrEFAtkDzNd2reuP')
    puppetMasterProgramId = new PublicKey('7UBgahynaRRc7j5qKm5d4NJCXwMssPjWvRshXu2WPKT9')
    
    puppetProgram = new anchor.Program(puppetIDL, puppetProgramId)
    puppetMasterProgram = new anchor.Program(puppetMasterIDL, puppetMasterProgramId)
  })

  it('should make a cross program invocation frm puppet master to puppet', async () => {
    const puppetAccount = await createPuppetAccount()

    await puppetMasterProgram.rpc.pullStrings(new anchor.BN(123), {
      accounts: {
        puppet: puppetAccount.publicKey,
        puppetProgram: puppetProgram.programId
      }
    })

    const account = await puppetProgram.account.state.fetch(puppetAccount.publicKey)
    expect(account.data.toNumber()).to.equal(123)
  })

  it('does not have any access control', async () => {
    const puppetAccount = await createPuppetAccount()
    await puppetProgram.rpc.setData(new anchor.BN(321), {
      accounts: {
        puppet: puppetAccount.publicKey
      },
      signers: [provider.wallet.payer]
    })

    const account = await puppetProgram.account.state.fetch(puppetAccount.publicKey)
    expect(account.data.toNumber()).to.equal(321)
  })

  it.only('should allow the CPI to go though the puppet master', async () => {
    const puppetAccount = await createPuppetAccount()
    const [pda] = await initializePda()

    await puppetMasterProgram.rpc.pullStringsAuth(new anchor.BN(123), {
      accounts: {
        puppet: puppetAccount.publicKey,
        puppetProgram: puppetProgram.programId,
        puppetMasterPda: pda
      }
    })

    const account = await puppetProgram.account.state.fetch(puppetAccount.publicKey)
    expect(account.data.toNumber()).to.equal(123)
  })

  it('should fail if invoked by unauthorized user which is not the puppet master pda', async () => {
    const puppetAccount = await createPuppetAccount()

    try {
      await puppetProgram.rpc.setDataAuth(new anchor.BN(123), {
        accounts: {
          puppet: puppetAccount.publicKey,
          authority: (await createPuppetMasterPDA())[0]
        },
        signers: [provider.wallet.payer]
      })
    } 
    catch(error) {
      expect(error.message).to.equal('Signature verification failed')
    }

    expect(true).to.equal(false)
  })

  it('should fail if invoked by unauthorized user that uses himself as the authority account', async () => {
    const puppetAccount = await createPuppetAccount()

    try {
      await puppetProgram.rpc.setDataAuth(new anchor.BN(123), {
        accounts: {
          puppet: puppetAccount.publicKey,
          authority: provider.wallet.publicKey
        },
        signers: [provider.wallet.payer]
      })
    } 
    catch(error) {
      expect(error.message).to.equal('6000: only puppet master')
    }

    expect(true).to.equal(false)
  })
})
