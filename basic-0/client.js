import anchor from '@project-serum/anchor'
import idl from './target/idl/basic_0.json'

// set the local provider
anchor.setProvider(anchor.Provider.local())

// address of the deployed program
const programId = new anchor.web3.PublicKey('eaqfh3zz5gPL8fczHC9tjYksrJwjU8RZ2BCCv1trXRU')

// create the program instance
const program = new anchor.Program(idl, programId)

// interact with the program via RPC calls
await program.rpc.initialize()
