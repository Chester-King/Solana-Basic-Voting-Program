
   
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
  const updateAccount = anchor.web3.Keypair.generate();

  
  it('Is initialized!', async () => {
    // Add your test here.
    const program = await anchor.workspace.VotingAnchorJs;
    await console.log("User in initializer",provider.wallet.publicKey.toString())
    await console.log("voteAccount ",voteAccount.publicKey.toString())
    await console.log("stateAccount",stateAccount.publicKey.toString())
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
    const account2 = await program.account.voteAccount.fetch(
      voteAccount.publicKey
    );
    await console.log(account2);
  });

  it('Vote for a proposal', async () => {
    // Add your test here.
    const program = await anchor.workspace.VotingAnchorJs;
    await console.log("Signer in voteOnProposal",provider.wallet.publicKey.toString())
    await console.log("Signer in voteOnProposal in BN form",provider.wallet.publicKey)
    await console.log("voteAccount ",voteAccount.publicKey.toString())
    await console.log("stateAccount",stateAccount.publicKey.toString())
    const tx = await program.rpc.voteOnProposal(
      "proposal1",
      {
        accounts: {
          state : stateAccount.publicKey,
          voteAccount : voteAccount.publicKey,
          signer : provider.wallet.publicKey,
        },
        // signers: [
        //   // stateAccount,
        //   // voteAccount,
        // ]
      }
    );
    
    await console.log("Your transaction signature", tx);
    const account = await program.account.state.fetch(
      stateAccount.publicKey
    );
    await console.log(account);
    const account2 = await program.account.voteAccount.fetch(
      voteAccount.publicKey
    );
    await console.log(account2);
  });
});
