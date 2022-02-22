
   
const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;
// import { VotingAnchorApp } from '../target/types/voting_anchor_app';

describe('voting-anchor-js', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const stateAccount = anchor.web3.Keypair.generate();
  const voteAccount = anchor.web3.Keypair.generate();

  
  it('Is initialized!', async () => {
    // Add your test here.
    const program = await anchor.workspace.VotingAnchorJs;
    const tx = await program.rpc.initialize(
      ["proposal1","proposal2","proposal3"],
      {
        accounts: {
          state : stateAccount.publicKey,
          user : provider.wallet.publicKey,
          voteAccount : voteAccount.publicKey,
          systemProgram : anchor.web3.SystemProgram.programId

        },
        signers: [
          stateAccount,
          voteAccount
        ]
      }
    );
    await console.log("Your transaction signature", tx);
    const account = await program.account.state.fetch(
      stateAccount.publicKey
    );
    await console.log(account);
  });
});
