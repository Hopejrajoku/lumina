import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import * as fs from "fs";

async function main() {
    // 1. Setup Environment
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // 2. Load the IDL directly from the target folder
    // This ensures the discriminator matches your local 'pub fn authorize_user'
    const idlPath = "./target/idl/lumina.json";
    if (!fs.existsSync(idlPath)) {
        console.error("❌ IDL not found! Run 'anchor build' first.");
        return;
    }
    const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));
    const program = new anchor.Program(idl, provider);

    console.log("--------------------------------------------------");
    console.log("Lumina Protocol: Institutional Compliance Check");
    console.log(`Program ID: ${program.programId.toString()}`);
    
    // 3. Setup Test User
    const testUser = Keypair.generate().publicKey;
    const countryCode = "NG"; 

    // Derive PDA: [b"identity", user_pubkey]
    const [identityPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("identity"), testUser.toBuffer()],
        program.programId
    );

    console.log(`Target User:  ${testUser.toString()}`);
    console.log(`Identity PDA: ${identityPda.toString()}`);
    console.log("--------------------------------------------------");

    try {
        console.log("Sending transaction...");
        
        // 4. Call the method
        // Anchor TS will automatically handle the 'authorize_user' -> 'authorizeUser' mapping
        const tx = await program.methods
            .authorizeUser(countryCode)
            .accounts({
                identityRecord: identityPda,
                user: testUser,
                admin: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        console.log("✅ Transaction successful!");
        console.log(`🔗 Explorer: https://explorer.solana.com/tx/${tx}?cluster=devnet`);

        // 5. Fetch and Verify
        const accountData: any = await program.account.identityRecord.fetch(identityPda);
        console.log(`📌 On-Chain Country: ${accountData.countryCode}`);
        console.log(`🛡️ Status: ${accountData.isVerified ? "VERIFIED" : "UNVERIFIED"}`);

    } catch (err: any) {
        console.error("❌ Execution Failed!");
        if (err.logs) {
            console.log("Program Logs:", err.logs);
        } else {
            console.error(err);
        }
    }
}

main();