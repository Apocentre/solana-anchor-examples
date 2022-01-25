import anchor from '@project-serum/anchor'
import {use, expect} from 'chai'
import chaiAsPromise from 'chai-as-promised'

use(chaiAsPromise)
const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe.only('multi-signers', () => {
  const provider = anchor.Provider.local()
  anchor.setProvider(provider)

  const program = anchor.workspace.SigVerify
  const authProvider = Keypair.generate()
  const stateAccount = Keypair.generate()


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

  it('should fail if signature is wrong', async () => {
    const partialTx = await program.transactions.contribute(nonce, sig, amount, {
      accounts: {
        state: stateAccount.publicKey,
        sender: provider.wallet.publicKey,
        authProvider: authProvider.publicKey,
      },
      signers: [provider.wallet.payer]
    })


  })
});
