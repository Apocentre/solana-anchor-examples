import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SigVerify } from '../target/types/sig_verify';

describe('sig-verify', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SigVerify as Program<SigVerify>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
