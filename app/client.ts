const anchor = require("@project-serum/anchor");
const web3 = require("@solana/web3.js");
const bs58 = require("@project-serum/anchor/dist/cjs/utils/bytes/bs58");
const BN = require("bn.js");
const spl = require("@solana/spl-token")

async function main() {

  const programId = new anchor.web3.PublicKey("9fnWiZpicj8MNHymzYiKpA9GtdxqKeV5p7AYevp9hzWF");

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

    // console.log(idl, 'idl');
    // console.log(program, 'program');
    
    

  // const mint = new anchor.web3.PublicKey("J6PXH6vJZhS8SNzVqathiRCLPwmsetAYQHSqwgadofxJ");
  // let price = new BN(web3.LAMPORTS_PER_SOL*2);
  // const AUCTION_SIGNER_SEEDS = "testhuehuehuetest";
  
  // const [auctionAccount, bump] = await web3.PublicKey.findProgramAddress(
  //   [
  //     Buffer.from("auction"),
  //     programId.toBuffer(),
  //     mint.toBuffer(),
  //     Buffer.from(AUCTION_SIGNER_SEEDS),
  //   ],
  //   programId
  // );

  // console.log("creator loaded successfully", creator.publicKey.toBase58());
  
  const fromAccount = new anchor.web3.PublicKey("5wbnW6cAoT2Rdwk5ecdgpCzoCp3g9rV7Vr5jNFAJBbCY");
  const fromTokenAccount = new anchor.web3.PublicKey("8QhuyEzMW6fuPjVXSpVr2d4Uneq5D9HKe38wS5zeDLoB");
  const toTokenAccount = new anchor.web3.PublicKey("7bgtStMLU7D5YxqpFNat1gHkzTqQVBzEJrJSyuTRNhcw");
  
  const tokenProgram = new anchor.web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    const tx = await program.methods.transferNft().accounts({
      fromAccount: wallet.publicKey,
      fromTokenAccount: fromTokenAccount,
      toTokenAccount: toTokenAccount,
      tokenProgram: tokenProgram,
    }).signers([]).rpc();

    // const tx = await program.rpc.transferNft({
    //   accounts: {
    //     fromAccount: fromAccount,
    //     fromTokenAccount: fromTokenAccount,
    //     toTokenAccount: toTokenAccount,
    //     tokenProgram: tokenProgram,
    //   },
    //   signers: []
    // });

  // console.log(spl.TOKEN_PROGRAM_ID.toBase58());
  

}

main()