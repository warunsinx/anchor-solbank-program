import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorSolbankProgram } from "../target/types/anchor_solbank_program";

describe("anchor-solbank-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .AnchorSolbankProgram as Program<AnchorSolbankProgram>;

  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("User Vault Account is Initted", async () => {
    await program.methods
      .initUserVault()
      .accounts({
        userVaultAccount: vaultPda,
      })
      .rpc();
  });
});
