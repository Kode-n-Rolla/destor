import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../target/types/destor";
import { expect } from "chai";
import { ensureProtocolInitialized } from "./helpers/protocol";

describe("destor::organization", () => {
    ///////////////////////////////
    //       Preparation         //
    ///////////////////////////////
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    const program = anchor.workspace.destor as Program<Destor>;

    const airdropSolAmount = 7;

    // Actors
    let admin: anchor.web3.Keypair;
    let authority: anchor.web3.Keypair;

    let protocolPda: anchor.web3.PublicKey;
    let organizationId: Buffer;
    let organizationPda: anchor.web3.PublicKey;
    let organizationBump: number;

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
    //           SetUp           //
    ///////////////////////////////
    before(async () => {
        admin = anchor.web3.Keypair.generate();

        await airdrop(admin.publicKey);

        const protocol = await ensureProtocolInitialized(program, admin);

        protocolPda = protocol.protocolPda;
    });

    beforeEach(async () => {
        authority = anchor.web3.Keypair.generate();

        await airdrop(authority.publicKey);

        organizationId = anchor.web3.Keypair.generate()
            .publicKey
            .toBuffer();

        const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("organization"),
                organizationId
            ],
            program.programId
        );

        organizationPda = pda;
        organizationBump = bump;
    })


    ///////////////////////////////
    //     Happy Path Tests      //
    ///////////////////////////////
    it.only("register an organization", async () => {
        const threshold = 2;

        await program.methods
            .registerOrganization(
                { manufacturer : {} },
                Array.from(organizationId),
                authority.publicKey,
                threshold
            )
            .accountsPartial({
                protocolConfig: protocolPda,
                organization: organizationPda,
                admin: admin.publicKey,
            })
            .signers([admin])
            .rpc();

        const organizationAcount = await program.account.organization.fetch(organizationPda);

        expect(organizationAcount.authority.equals(authority.publicKey)).to.eq(true);
        expect(organizationAcount.pendingAuthority.equals(anchor.web3.PublicKey.default)).to.eq(true);
        expect(Buffer.from(organizationAcount.organizationId).equals(organizationId)).to.eq(true);
        expect(organizationAcount.threshold).to.eq(threshold);
        expect(organizationAcount.active).to.eq(true);
        expect(organizationAcount.bump).to.eq(organizationBump);
    });
});