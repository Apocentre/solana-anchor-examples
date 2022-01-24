import anchor from '@project-serum/anchor'
import borsh from 'borsh'
import nacl from 'tweetnacl'
import {use, expect} from 'chai'
import chaiAsPromise from 'chai-as-promised'
import sigVerifyIDL from '../target/idl/sig_verify.json'

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

  it('should initialize', async () => {
    await initialize()

    const account = await program.account.state.fetch(stateAccount.publicKey)
    expect(account.authority.toString()).to.equal(authority.publicKey.toString())
  });

  it('should fail if signature is wrong', async () => {
    class ContributeMsg {
      constructor(program, nonce, sender, amount) {
        this.program = program
        this.nonce = nonce
        this.sender = sender
        this.amount = amount
      }
    }

    const type = {
      "kind": "struct",
      "fields": [
        ['program', [32]],
        ['nonce', 'u64'],
        ['sender', [32]],
        ['amount', 'u64']
      ]
    }
    
    const nonce = new anchor.BN(100)
    const amount = new anchor.BN(5000)
    const value = new ContributeMsg(
      program.programId.toBytes(),
      nonce,
      provider.wallet.publicKey.toBytes(),
      amount,
    )
    const schema = new Map([[ContributeMsg, type]]);

    const message = borsh.serialize(schema, value)
    const sig = nacl.sign.detached(message, authority.secretKey) 

    console.log('message ---> ', message)
    console.log('Sig ---> ', sig)

    await program.rpc.contribute(nonce, sig, amount, {
      accounts: {
        state: stateAccount.publicKey,
        sender: provider.wallet.publicKey,
      },
      signers: [provider.wallet.payer]
    })
  })
});
