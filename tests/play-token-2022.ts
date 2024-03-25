import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { PlayToken2022 } from "../target/types/play_token_2022";
import {
  getAssociatedTokenAddressSync,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { min } from "bn.js";
describe("play-token-2022", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PlayToken2022 as Program<PlayToken2022>;
  const provider = anchor.getProvider();
  const wallet = anchor.Wallet.local();
  console.log("wallet", wallet.publicKey.toBase58());
  const mint = web3.Keypair.generate();
  const [vault] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), mint.publicKey.toBuffer()],
    program.programId
  );
  let vaut_ata: web3.PublicKey;
  let userAta: web3.PublicKey;
  before(async () => {
    vaut_ata = await getAssociatedTokenAddressSync(
      mint.publicKey,
      vault,
      true,
      TOKEN_2022_PROGRAM_ID
    );
    userAta = await getAssociatedTokenAddressSync(
      mint.publicKey,
      wallet.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );
  });
  it("Is initialized!", async () => {
    // Add your test here.

    const tx = await program.methods
      .initialize()
      .accounts({
        signer: wallet.publicKey,
        mint: mint.publicKey,
        vault,
        vaultAta: vaut_ata,
        systemProgram: web3.SystemProgram.programId,
        token2022Program: TOKEN_2022_PROGRAM_ID,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([mint, wallet.payer])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should transfer tokens", async () => {
    const tx = await program.methods
      .transfer(new anchor.BN(100 * 10 ** 9))
      .accounts({
        signer: wallet.publicKey,
        mint: mint.publicKey,
        vault,
        vaultAta: vaut_ata,
        userAta: userAta,
        systemProgram: web3.SystemProgram.programId,
        token2022Program: TOKEN_2022_PROGRAM_ID,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([wallet.payer])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
