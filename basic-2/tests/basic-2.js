import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Basic2 } from '../target/types/basic_2';

describe('basic-2', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Basic2 as Program<Basic2>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
