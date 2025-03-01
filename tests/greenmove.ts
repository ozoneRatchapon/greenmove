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


});
