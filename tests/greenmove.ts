import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Greenmove } from "../target/types/greenmove";
import { assert } from "chai";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { randomBytes } from "crypto";
// import { getMinimumBalanceForRentExemption } from "@solana/spl-token";
import { log } from "console";

describe("greenmove", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Greenmove as Program<Greenmove>;
  const provider = anchor.getProvider();

  // const user = anchor.web3.Keypair.generate();
  const signer = anchor.web3.Keypair.generate();
  const seed = new anchor.BN(randomBytes(8));
  const [user_account_state] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("state"),
      signer.publicKey.toBuffer(),
      // seed.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );
  const systemProgram = SystemProgram.programId;

  it("airdrop", async () => {
    let tx = new anchor.web3.Transaction();
    tx.instructions = [
      SystemProgram.transfer({
        fromPubkey: program.provider.publicKey,
        toPubkey: signer.publicKey,
        lamports: 0.2 * LAMPORTS_PER_SOL,
      }),
      SystemProgram.transfer({
        fromPubkey: program.provider.publicKey,
        toPubkey: user_account_state,
        lamports: 0.2 * LAMPORTS_PER_SOL,
      }),

    ];

    console.log({ signer: signer.publicKey.toString(), user_account_state: user_account_state.toString() });
    const txSig = await provider.sendAndConfirm(tx);
    console.log("Your transaction signature", txSig);

  });

  it("Creates a new user account", async () => {

    // Ensure accounts are initialized and valid before the test
    assert.ok(signer.publicKey);
    assert.ok(user_account_state);
    assert.ok(systemProgram);

    const displayName = "ozonesk"; // Example display name
    const location = "Thailand"; // Example location

    const accounts = {
      signer: signer.publicKey,
      userAccountState: user_account_state,
      systemProgram: systemProgram,
    };

    try {
      const tx = await program.methods
        .createUser(seed, displayName, location)
        .accounts({ ...accounts })
        .signers([signer])
        .rpc();
      console.log("Your transaction signature", tx);

      const account = await program.account.userAccountState.fetch(user_account_state); //Make sure that the account name matches the one in your program
      // assert is used to check if the account is created successfully
      assert.ok(account, "Account should exist");
      console.log("Account", account);
      assert.equal(account.displayName, displayName, "Display name should match");
      assert.equal(account.location, location, "Location should match");

    } catch (error) {
      console.error("Transaction failed with error:", error);
      assert.fail("Transaction should not have failed");
    }
  });

  it("Updates an account", async () => {
    const displayName = "ozonesk2"; // Example display name
    const location = "Thailand2"; // Example location

    const accounts = {
      signer: signer.publicKey,
      userAccountState: user_account_state,
    };

    try {
    const tx = await program.methods
    .updateUserProfile(displayName, location)
    .accounts({ ...accounts })
    .signers([signer])
    .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.userAccountState.fetch(user_account_state);
    console.log("Update Account", account);
    assert.equal(account.displayName, displayName);
    assert.equal(account.location, location);
    } catch (error) {
      console.error("Error updating account:", error);
      assert.fail("Transaction should not have failed");
    }
  });

  it("Creates a new community leader account", async () => {
    const seed = new anchor.BN(1); // Example seed value
    const displayName = "exampleDisplayName"; // Example display name
    const location = "Thailand"; // Example location
    const tx = await program.methods.createCommunityLeader(seed, displayName, location).rpc();
    console.log("Your transaction signature", tx);

    const [leaderAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("community_leader"),
        anchor.AnchorProvider.env().wallet.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // const account = await program.account.communityLeader.fetch(leaderAccount);
    // assert.equal(account.displayName, displayName);
    // assert.equal(account.location, location);
  });

  it("Log Action", async () => {
    const actionType = "exampleAction"; // Example action
    const amount = new anchor.BN(150); // Example amount
    const timestamp = new anchor.BN(Date.now()); // Example timestamp
    const location = "Thailand"; // Example location
    const proof = "exampleProof"; // Example proof

    try {
      const tx = await program.methods.logAction(actionType, amount, timestamp, location, proof).rpc();
      console.log("Your transaction signature", tx);

      const [actionLogAccount] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("action_log"),
          anchor.AnchorProvider.env().wallet.publicKey.toBuffer(),
          timestamp.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      const log = await program.account.actionLog.fetch(actionLogAccount);
      assert.equal(log.actionType, actionType);
      assert.equal(log.amount.toString(), amount.toString());
      assert.equal(log.location, location);
      assert.equal(log.proof, proof);
    } catch (error) {
      console.error("Error logging action:", error);
    }
  });

  it("get user history", async () => {
    const seed = new anchor.BN(1); // Example seed value
    const [userAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("user"),
        anchor.AnchorProvider.env().wallet.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    try {
      const history = await program.account.actionLog.fetch(userAccount);
      console.log("User history", history);
      assert.isNotNull(history);
    } catch (error) {
      console.error("Error fetching user history:", error);
    }
  });

  it("Community leader create quest", async () => {
    const seed = new anchor.BN(1); // Example seed value
    const questName = "exampleQuest"; // Example quest name
    const description = "exampleDescription"; // Example description
    const reward = new anchor.BN(100); // Example reward
    const conditions = "exampleConditions"; // Example conditions
    const deadline = new anchor.BN(Date.now() + 100000); // Example deadline
    const targetAudience = "exampleAudience"; // Example target audience

    const tx = await program.methods.createQuest(questName, description, conditions, reward, deadline, targetAudience).rpc();
    console.log("Your transaction signature", tx);

    const [questAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("quest"),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // const quest = await program.account.quest.fetch(questAccount);
    // assert.equal(quest.name, questName);
    // assert.equal(quest.description, description);
    // assert.equal(quest.reward.toString(), reward.toString());
    // assert.equal(quest.conditions, conditions);
    // assert.equal(quest.deadline.toString(), deadline.toString());
    // assert.equal(quest.targetAudience, targetAudience);
  });

  it("user join quest", async () => {
    const seed = new anchor.BN(1); // Example seed value
    const [questPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("quest"),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const tx = await program.methods.joinQuest(questPda)
      .accounts({
        questAccount: questPda,
        user: anchor.AnchorProvider.env().wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const quest = await program.account.quest.fetch(questPda);
    assert.isTrue(quest.participants.includes(anchor.AnchorProvider.env().wallet.publicKey));
  });
});