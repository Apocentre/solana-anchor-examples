import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Basic0 } from '../target/types/basic_0';

describe('basic-0', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Basic0 as Program<Basic0>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
