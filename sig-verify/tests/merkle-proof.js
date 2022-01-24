import anchor from '@project-serum/anchor'
import borsh from 'borsh'
import {use, expect} from 'chai'
import chaiAsPromise from 'chai-as-promised'
import {createTree, getRoot} from './merkle-tree.js'

use(chaiAsPromise)
const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe.only('merkle-proof', () => {
  const provider = anchor.Provider.local()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider)

  const program = anchor.workspace.MerkleProof
  const stateAccount = Keypair.generate()
  const participants = Array.from(new Array(100), () => Keypair.generate().publicKey)
  const merkleTree = createTree(participants)

  const initialize = async () => {
    await program.rpc.initialize(getRoot(merkleTree), {
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
    expect(getRoot(merkleTree).toString('hex')).to.equal(account.merkleRoot.toString('hex'))
  });

  it('should fail if address is not in the merkle tree', async () => {
  
  })
});
