import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { YourProgram } from "../target/types/studibet";

describe("studibet", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.YourProgram as Program<YourProgram>;

  it("Creates a bet!", async () => {
    const betSeed = new anchor.BN(12345); // Unique seed for the bet
    const user = provider.wallet.publicKey;

    // PDA for the bet account
    const [betPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("bet"), user.toBuffer(), betSeed.toArrayLike(Buffer)],
      program.programId
    );

    // Transaction to create the bet
    await program.methods
      .createBet(
        betSeed,
        new anchor.BN(Date.now() / 1000 + 60), // open_until: 1 minute from now
        new anchor.BN(Date.now() / 1000 + 3600), // resolve_date: 1 hour from now
        300, // average_grade_prediction (e.g., 3.00 scaled by 100)
        provider.wallet.publicKey // Resolver
      )
      .accounts({
        bet: betPda,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Bet account created:", betPda.toBase58());
  });
});
