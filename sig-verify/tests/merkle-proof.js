import anchor  from '@project-serum/anchor'
import {use, expect} from 'chai'
import chaiAsPromise from 'chai-as-promised'
import {createTree, getRoot, createLeaf, getProof, getProofIndices} from './merkle-tree.js'

use(chaiAsPromise)
const {SystemProgram, PublicKey, Keypair} = anchor.web3

describe('merkle-proof', () => {
  const provider = anchor.Provider.local()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider)

  const program = anchor.workspace.MerkleProof
  const stateAccount = Keypair.generate()
  const participants = Array.from(new Array(100), () => Keypair.generate())
  const merkleTree = createTree(participants.map(p => p.publicKey))

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

  it('should revert if a non whitelisted account uses other whitelisted account proof', async () => {
    const proof = getProof(merkleTree, createLeaf(participants[0].publicKey))
    const nonWhitelistedAccount = Keypair.generate()

    console.log('>>>>>>>>>>>>>', proof)

    await program.rpc.contribute(proof, new anchor.BN(500), {
      accounts: {
        state: stateAccount.publicKey,
        sender: nonWhitelistedAccount.publicKey
      },
      signers: [nonWhitelistedAccount]
    })
  })
});
