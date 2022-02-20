
   
const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;
// import { VotingAnchorApp } from '../target/types/voting_anchor_app';

describe('voting-anchor-js', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const stateAccount = anchor.web3.Keypair.generate();

  
  it('Is initialized!', async () => {
    // Add your test here.
    const program = await anchor.workspace.VotingAnchorJs;
    const tx = await program.rpc.initialize({
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
      state: stateAccount.publicKey,
    });
    console.log("Your transaction signature", tx);

  });
});
