import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../target/types/destor";
import { expect } from "chai";

describe("destor", () => {
    ///////////////////////////////
    //       Preparation         //
    ///////////////////////////////
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.destor as Program<Destor>;

    const airdropSolAmount = 5;

    // Actors
    let alice: anchor.web3.Keypair;
    //let bob: anchor.web3.Keypair;
    //let charlie: anchor.web3.Keypair;

    // Airdrop function
    const airdrop = async (
        pubkey: anchor.web3.PublicKey,
        sol = airdropSolAmount
    ) => {
        const sig = await provider.connection.requestAirdrop(
            pubkey,
            sol * anchor.web3.LAMPORTS_PER_SOL
        );

        const latest = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction(
            {
                signature: sig,
                blockhash: latest.blockhash,
                lastValidBlockHeight: latest.lastValidBlockHeight
            },
            "confirmed"
        );
    };

    ///////////////////////////////
    //    Foundry-like setUp     //
    ///////////////////////////////
    beforeEach(async () => {
        alice = anchor.web3.Keypair.generate();

        await Promise.all([
            airdrop(alice.publicKey)
        ]);
    });

    ///////////////////////////////
    //     Happy Path Tests      //
    ///////////////////////////////
    it("create a protocol", async () => {
        const [protocolPda, protocolBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("protocol_config")],
            program.programId
        );

        await program.methods
            .initialize()
            .accountsPartial({
                admin: alice.publicKey,
                protocolConfig: protocolPda
            })
            .signers([alice])
            .rpc();

        const protocolAccount = await provider.connection.getAccountInfo(protocolPda);
        const protocolConfig = await program.account.protocolConfig.fetch(protocolPda);

        // Assertions
        expect(protocolAccount).to.be.not.null;
        expect(protocolConfig.admin.equals(alice.publicKey)).to.eq(true);
        expect(protocolConfig.bump).to.eq(protocolBump);
    });
});