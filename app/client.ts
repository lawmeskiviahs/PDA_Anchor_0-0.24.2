const anchor = require("@project-serum/anchor");
const web3 = require("@solana/web3.js");
const bs58 = require("@project-serum/anchor/dist/cjs/utils/bytes/bs58");
const BN = require("bn.js");
const spl = require("@solana/spl-token")

async function main() {

  const programId = new anchor.web3.PublicKey("CFNwgx5rdMbDtdBcDztcCzPeviMCtH8D1yFaLVTi1khL");
  const SystemProgram = new anchor.web3.PublicKey("11111111111111111111111111111111");

  const idl = JSON.parse(
    require("fs").readFileSync("../target/idl/myprogram.json", "utf8")
  );

  const creator = web3.Keypair.fromSecretKey(
    bs58.decode("Py2R2BgK9XVj5CVMUzwpgsX7c52fjDuwUojGdvVpZUdkDgMR5RbN6JnWsE68mkYLEUkAP13JJTcFGYDXh6Zm5zk")
  );

  // const connection = new web3.Connection("https://api.devnet.solana.com","confirmed");
  const connection = new web3.Connection("http://localhost:8899","confirmed");
  let wallet = new anchor.Wallet(creator);
  let provider = new anchor.AnchorProvider(connection,wallet,anchor.AnchorProvider.defaultOptions());

  let program = new anchor.Program(
    idl,
    programId,
    provider,
  );
  
  const associatedTokenProgram = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
  // const mint = new anchor.web3.PublicKey("3ZzBu6LBmveypWcFcUmSYvMTi4NtYDdcsNEitniyEAky");
  const mint = new anchor.web3.PublicKey("4m6bGiEYyvdYVcdio2LDdiNe2wuddsp9dyA5ntsjsvLF");
  const toAccount = new anchor.web3.PublicKey("FLtJXfybBXBUPYwzNFgHuAJJaDjoPzvuPq9ZcKPhLsxc");
  const fromTokenAccount = new anchor.web3.PublicKey("CV6RB9Qy8XgbCEwtp9gdVMZMzenjXz26m2uhLz75gPp9");
  const toTokenAccount = await anchor.web3.PublicKey.findProgramAddress(
    [
      (wallet.publicKey).toBuffer(), 
      programId.toBuffer(), 
      mint.toBuffer()
    ],
    associatedTokenProgram
);

  const tokenProgram = new anchor.web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    const tx = await program.methods.transferNft().accounts({
      fromAccount: wallet.publicKey,
      fromTokenAccount: fromTokenAccount,
      toAccount: toAccount,
      toTokenAccount: toTokenAccount,
      tokenProgram: tokenProgram,
      mint: mint,
      associatedTokenProgram: associatedTokenProgram,
      systemProgram: SystemProgram,
    }).signers([]).rpc();

}

main()