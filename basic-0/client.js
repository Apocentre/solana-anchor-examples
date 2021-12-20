import anchor from '@project-serum/anchor'
import idl from './target/idl/basic_0.json'

// set the local provider
anchor.setProvider(anchor.Provider.local())

// address of the deployed program
const programId = new anchor.web3.PublicKey('4mvYYUkSizk9RkCD9SRKXjAo4ckncvQN3fKYA16KVxWy')

// create the program instance
const program = new anchor.Program(idl, programId)

// interact with the program via RPC calls
await program.rpc.initialize()
