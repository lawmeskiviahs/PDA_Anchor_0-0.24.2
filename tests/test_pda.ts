import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestPda } from "../target/types/test_pda";

describe("test_pda", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TestPda as Program<TestPda>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
