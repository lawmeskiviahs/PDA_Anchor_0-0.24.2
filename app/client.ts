
const anchor = require("@project-serum/anchor");
const web3 = require("@solana/web3.js");
const bs58 = require("@project-serum/anchor/dist/cjs/utils/bytes/bs58");
const BN = require("bn.js")

async function main() {

  const programId = new anchor.web3.PublicKey("61reie38A5ecZQ45ebeeCcQgBQ82NtA7h59jPLCzx6mK");

  const idl = JSON.parse(
    require("fs").readFileSync("../target/idl/myprogram.json", "utf8")
  );


  const creator = web3.Keypair.fromSecretKey(
    bs58.decode("Py2R2BgK9XVj5CVMUzwpgsX7c52fjDuwUojGdvVpZUdkDgMR5RbN6JnWsE68mkYLEUkAP13JJTcFGYDXh6Zm5zk")
  );

  const connection = new web3.Connection(web3.clusterApiUrl("devnet"),"confirmed");
    let wallet = new anchor.Wallet(creator)
    let provider = new anchor.AnchorProvider(connection,wallet,anchor.AnchorProvider.defaultOptions())

  let program = new anchor.Program(
    idl,
    programId,
    provider,
  );

  const mint = new anchor.web3.PublicKey("J6PXH6vJZhS8SNzVqathiRCLPwmsetAYQHSqwgadofxJ");
  let price = new BN(web3.LAMPORTS_PER_SOL*2);
  const AUCTION_SIGNER_SEEDS = "testhuehuehuetest";
  
  const [auctionAccount, bump] = await web3.PublicKey.findProgramAddress(
    [
      Buffer.from("auction"),
      programId.toBuffer(),
      mint.toBuffer(),
      Buffer.from(AUCTION_SIGNER_SEEDS),
    ],
    programId
  );

  console.log("creator loaded successfully", creator.publicKey.toBase58());  
    const tx = await program.methods.createAuction(price, bump).accounts({
      auctionAccount: auctionAccount,
      seller: creator.publicKey,
      mint: mint,
      systemProgram: web3.SystemProgram.programId,
    }).signers([creator]).rpc();

  console.log(program);
  

}

main()