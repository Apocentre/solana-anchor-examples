import anchor from '@project-serum/anchor'
import {use, expect} from 'chai'
import chaiAsPromise from 'chai-as-promised'
// import sigVerifyIDL from '../target/idl/sig.json'

use(chaiAsPromise)
const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe('sig-verify', () => {
  const provider = anchor.Provider.local()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider)

  const program = anchor.workspace.SigVerify
  const authority = Keypair.generate()
  const stateAccount = Keypair.generate()

  const initialize = async () => {
    await program.rpc.initialize(authority.publicKey, {
      accounts: {
        state: stateAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [stateAccount, provider.wallet.payer]
    })
  }

  it('Is initialized!', async () => {
    await initialize()

    const account = await program.account.state.fetch(stateAccount.publicKey)
    expect(account.authority.toString()).to.equal(authority.publicKey.toString())
  });
});
