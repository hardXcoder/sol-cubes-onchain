import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaCubes } from "../target/types/solana_cubes";
import * as assert from "assert";
import * as bs58 from "bs58";

describe("solanacubes", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaCubes as Program<SolanaCubes>;

  it("Placing a bet on heads!", async () => {
    const betAccount = anchor.web3.Keypair.generate();

    await program.rpc.placeBet(new anchor.BN(1), false, {
      accounts: {
        bet: betAccount.publicKey,
        author: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [betAccount],
    });

    // Fetch the account details of the created tweet.
    const bet = await program.account.bet.fetch(betAccount.publicKey);

    assert.equal(bet.isWin, false);
  });

  it("Placing a bet on tail!", async () => {
    const betAccount = anchor.web3.Keypair.generate();

    await program.rpc.placeBet(new anchor.BN(1), true, {
      accounts: {
        bet: betAccount.publicKey,
        author: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [betAccount],
    });

    // Fetch the account details of the created tweet.
    const bet = await program.account.bet.fetch(betAccount.publicKey);

    assert.equal(bet.isWin, true);
  });
});
