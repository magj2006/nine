import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";
import * as ed from "@noble/ed25519";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import {
  ComputeBudgetProgram,
  LAMPORTS_PER_SOL,
  Transaction,
} from "@solana/web3.js";
import BN from "bn.js";
import { assert } from "chai";
import {NineDragons} from "../target/types/nine_dragons"
import {
  project1Creator,
  project2Creator,
  recipient1,
  user1,
  user2,
} from "../.env/test-users";
// import * as blake3 from 'blake3';

describe("Origin Point", () => {
  // const provider = anchor.AnchorProvider.env();
  // const provider = anchor.AnchorProvider.local("https://api.testnet.solana.com", {skipPreflight: true});
  // const provider = anchor.AnchorProvider.local("https://api.devnet.solana.com", {skipPreflight: true});
  // const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899", {skipPreflight: true});
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);
  // const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.NineDragons as anchor.Program<NineDragons>;

  const wallet = anchor.Wallet.local().payer;

  // The metadata for our NFT
  const createParam = {
    name: "Homer NFT",
    symbol: "HOMR",
    uri: "https://gw3.io/ipfs/QmVabVUWCRNSNPo66fnzEtQE4b2eWfE5uz7TMN6BsdBuqZ",
    project_name: "Homer NFT",
    signature: null,
    size: 100,
  };

  const id = 1;
  let signature: Uint8Array;

  const [protocolAccount] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("protocol")],
    program.programId
  );

  const [project1Account] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("project"),
      project1Creator.publicKey.toBuffer(),
      // new BN(id).toBuffer("le", 2),
      Buffer.from(createParam.name),
    ],
    program.programId
  );

  console.log(`id: ${new BN(id).toBuffer("le", 2)}`);
  console.log(`project1Account: ${project1Account}`);

  // const mintCollection = new Keypair();
  const [mintCollection] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("mint"),
      project1Account.toBuffer(),
    ],
    program.programId
  );

  const [update_authority] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authority"),
      protocolAccount.toBuffer(),
      Buffer.from(createParam.name),
    ],
    program.programId
  );

  const requireAirdrop = async () => {
    const lamports = 100 * LAMPORTS_PER_SOL;
    await program.provider.connection
      .requestAirdrop(project1Creator.publicKey, lamports)
      .catch((e) => console.log(e));
    await program.provider.connection.requestAirdrop(
      project2Creator.publicKey,
      lamports
    );
    await program.provider.connection.requestAirdrop(user1.publicKey, lamports);
    await program.provider.connection.requestAirdrop(user2.publicKey, lamports);
    await program.provider.connection.requestAirdrop(
      recipient1.publicKey,
      lamports
    );
    // await program.provider.connection.requestAirdrop(
    //     update_authority.publicKey,
    //     lamports
    // );
  };

  before("Before", async () => {
    await requireAirdrop().catch((e) => console.log(e));
  });

  it("Initialize Project", async () => {
    const price = 2100000000;
    const sellerFeeBasisPoints = 5;

    console.log(`owner: ${project1Creator.publicKey}`);
    console.log(`config: ${project1Account}`);
    console.log(`recipient: ${recipient1.publicKey}`);
    console.log(`protocol: ${protocolAccount}`);
    // console.log(`updateAuthority: ${update_authority.publicKey}`);

    const isMutable = true;

    const tx = await program.methods
      .initProject(new BN(price), sellerFeeBasisPoints, isMutable)
      .accounts({
        owner: project1Creator.publicKey,
        recipient: recipient1.publicKey,
        operator: project1Creator.publicKey,
        // updateAuthority: update_authority.publicKey,
      })
      .signers([project1Creator])
      .rpc({ commitment: "confirmed", preflightCommitment: "confirmed" })
      .catch((e) => console.log(e));

    console.log(`Initialize project transaction signature: ${tx}`);

    const project1Config = await program.account.project.fetch(project1Account);
    assert.isTrue(
      project1Config.price.eq(new BN(price)),
      "init project failed 0"
    );
    assert.equal(
      project1Config.sellerFeeBasisPoints,
      sellerFeeBasisPoints,
      "init project failed 1"
    );
  });
  it("Create Collection!", async () => {
    let project1Config = await program.account.project.fetch(project1Account);
    assert.equal(project1Config.projectName, createParam.name, "Not found");

    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
      // mintCollection.publicKey,
      mintCollection,
      project1Creator.publicKey
    );

    let tx = new Transaction();

    tx.add(ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 }));

    const mintCollectionInstruction = await program.methods
      .createCollection({
        name: createParam.name,
        symbol: createParam.symbol,
        uri: createParam.uri,
      })
      .accounts({
        payer: project1Creator.publicKey,
        collectionTokenAccount: associatedTokenAccountAddress,
        // authority: update_authority.publicKey,
        recipient: recipient1.publicKey,
        originalOwner: project1Creator.publicKey,
      })
      .instruction();

    tx.add(mintCollectionInstruction);

    const transactionSignature = await provider.connection.sendTransaction(
      tx,
      [project1Creator],
      {
        preflightCommitment: "confirmed",
      }
    );

    console.log(
      `create collection transaction signature: ${transactionSignature}`
    );

    project1Config = await program.account.project.fetch(project1Account);
    assert.isTrue(
      project1Config.nonce.eq(new BN(1)),
      "create collection failed"
    );
  });
  it("Create an NFT!", async () => {
    const data = createParam.name
      .concat(":")
      .concat(createParam.symbol)
      .concat(":")
      .concat(createParam.uri);

    // Generate a keypair to use as the address of our mint account
    // const mintKeypair = new Keypair();
    const [mint] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("mint"),
        project1Account.toBuffer(),
        new BN(1).toBuffer("le", 8),
      ],
      program.programId
    );

    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
      // mintKeypair.publicKey,
      mint,
      user1.publicKey
    );

    let tx = new Transaction();

    tx.add(ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 }));

    const mintNftInstruction = await program.methods
      // .createNft(createParam.name, metadata, Buffer.from(signature).toString('hex'))
      .createNft({
        name: createParam.name,
        symbol: createParam.symbol,
        uri: createParam.uri,
        code: [],
      })
      .accounts({
        payer: user1.publicKey,
        collection: mintCollection,
        nftTokenAccount: associatedTokenAccountAddress,
        // authority: update_authority.publicKey,
        recipient: recipient1.publicKey,
        originalOwner: project1Creator.publicKey,
      })
      .instruction();

    tx.add(mintNftInstruction);

    try {
      const transactionSignature = await provider.connection.sendTransaction(
        tx,
        [user1],
        {
          preflightCommitment: "confirmed",
        }
      );

      console.log(`create nft transaction signature: ${transactionSignature}`);

      const project1Config = await program.account.project.fetch(
        project1Account
      );
      assert.isTrue(project1Config.nonce.eq(new BN(2)), "create nft failed");
    } catch (err) {
      console.log(`error: ${err}`);
    }
  });
  it("print protocol", async () => {
    const project = await program.account.project.fetch(project1Account);
    console.log(
      `project: ${project1Account}, ${project.owner}, ${project.pendingOwner}, ${project.projectName}, ${project.nonce}, ${project.padding}`
    );
  });
});
