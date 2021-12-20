import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Cpi } from '../target/types/cpi';

describe('cpi', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Cpi as Program<Cpi>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
