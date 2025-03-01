import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Greenmove } from "../target/types/greenmove";

describe("greenmove", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Greenmove as Program<Greenmove>;

  it("Creates a new account", async () => {
    // Add your test here.
    const seed = new anchor.BN(1); // Example seed value
    const displayName = "exampleDisplayName"; // Example display name
    const location = "Thailand"; // Example location
    const tx = await program.methods.createUser(seed, displayName, location).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Updates an account", async () => {
    // Add your test here.
    const seed = new anchor.BN(1); // Example seed value
    const displayName = "updatedDisplayName"; // Example display name
    const location = "Thailand2"; // Example location
    const tx = await program.methods.updateUserProfile(seed, displayName, location).rpc(); // Replace with the correct method name
    console.log("Your transaction signature", tx);
  });

  it("Creates a new community leader account", async () => {
    // Add your test here.
    const seed = new anchor.BN(1); // Example seed value
    const displayName = "exampleDisplayName"; // Example display name
    const location = "Thailand"; // Example location
    const tx = await program.methods.createCommunityLeader(seed, displayName, location).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Log Action", async () => {
    // Add your test here.

    const actionType = "exampleAction"; // Example action
    const amount = new anchor.BN(150); // Example amount
    const timestamp = new anchor.BN(Date.now()); // Example timestamp
    const location = "Thailand"; // Example location
    const proof = "exampleProof"; // Example proof

    try {
      const tx = await program.methods.logAction(actionType, amount, timestamp, location, proof).rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.error("Error logging action:", error);
    }
  });

  it("get user history", async () => {
    // Add your test here.
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
    } catch (error) {
      console.error("Error fetching user history:", error);
    }
  });

  it("Community leader create quest", async () => {
    // Add your test here.
    const seed = new anchor.BN(1); // Example seed value
    const questName = "exampleQuest"; // Example quest name
    const description = "exampleDescription"; // Example description
    const reward = new anchor.BN(100); // Example reward
    const conditions = "exampleConditions"; // Example conditions
    const deadline = new anchor.BN(Date.now() + 100000); // Example deadline
    const targetAudience = "exampleAudience"; // Example target audience

    // Fetch the community leader account
    const [communityLeaderPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("community_leader"),
        anchor.AnchorProvider.env().wallet.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Check if the community leader account exists
    try {
      await program.account.communityLeader.fetch(communityLeaderPda);
    } catch (error) {
      // If the community leader account does not exist, create it
      const displayName = "exampleDisplayName"; // Example display name
      const location = "Thailand"; // Example location
      await program.methods.createCommunityLeader(seed, displayName, location).rpc();
    }

    // Allocate space for the quest account
    const questAccount = anchor.web3.Keypair.generate();
    const tx = await program.methods.createQuest(questName, description, conditions, reward, deadline, targetAudience)
      .accounts({
        quest: questAccount.publicKey,
        communityLeader: communityLeaderPda,
        user: anchor.AnchorProvider.env().wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([questAccount])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("user join quest", async () => {
    // Add your test here.
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
  });
});
