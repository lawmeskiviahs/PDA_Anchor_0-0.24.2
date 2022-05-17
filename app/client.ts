
const anchor = require("@project-serum/anchor");
const web3 = require("@solana/web3.js");
const bs58 = require("@project-serum/anchor/dist/cjs/utils/bytes/bs58");

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

  const [baseAccountPDA, baseAccountPDABump] = await web3.PublicKey.findProgramAddress(
    [Buffer.from("blablahuehue")],
    programId
  );

  console.log("creator loaded successfully", creator.publicKey.toBase58());  
    let tx = await program.methods.fetch(baseAccountPDABump).accounts( {
      baseAccount: baseAccountPDA,
    }).signers([creator]).rpc();

}

main()