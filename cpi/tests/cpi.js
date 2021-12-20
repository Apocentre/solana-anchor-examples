import anchor from '@project-serum/anchor'
import {expect} from 'chai'
import puppetIDL from '../target/idl/puppet.json'
import puppetMasterIDL from '../target/idl/puppet_master.json'

const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe('cpi', () => {
  const provider = anchor.Provider.local()
  let puppetProgram
  let puppetMasterProgram

  const createPuppetAccount = async () => {
    const puppetAccount = Keypair.generate()

    await puppetProgram.rpc.initialize({
      accounts: {
        puppet: puppetAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [puppetAccount, provider.wallet.payer]
    })
    
    return puppetAccount
  }

  before(() => {
    anchor.setProvider(provider)

    const puppetProgramId = new PublicKey('7upcW754B7BSCJMZj5Vts3VFtbRYPrEFAtkDzNd2reuP')
    const puppetMasterProgramId = new PublicKey('7UBgahynaRRc7j5qKm5d4NJCXwMssPjWvRshXu2WPKT9')
    
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
})
