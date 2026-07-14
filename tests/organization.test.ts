import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../target/types/destor";
import { expect } from "chai";
import { ensureProtocolInitialized } from "./helpers/protocol";
import { testAdmin, airdrop } from "./helpers/actors";
import { Roles } from "./helpers/roles";
import { CreateOrganization } from "./helpers/organization";

describe("destor::organization", () => {
    ///////////////////////////////
    //       Preparation         //
    ///////////////////////////////
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    const program = anchor.workspace.destor as Program<Destor>;

    const airdropSolAmount = 7;

    // Actors
    //let admin: anchor.web3.Keypair;
    let authority: anchor.web3.Keypair;

    let protocolPda: anchor.web3.PublicKey;
    let organizationId: Buffer;
    let organizationPda: anchor.web3.PublicKey;
    let organizationBump: number;

    ///////////////////////////////
    //           SetUp           //
    ///////////////////////////////
    before(async () => {
        await airdrop(provider, testAdmin.publicKey, airdropSolAmount);

        const protocol = await ensureProtocolInitialized(program, testAdmin);

        protocolPda = protocol.protocolPda;
    });

    beforeEach(async () => {
        authority = anchor.web3.Keypair.generate();

        await airdrop(provider, authority.publicKey, airdropSolAmount);

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
    it("register an organization", async () => {
        const threshold = 2;

        await program.methods
            .registerOrganization(
                Roles.manufacturer,
                Array.from(organizationId),
                authority.publicKey,
                threshold
            )
            .accountsPartial({
                protocolConfig: protocolPda,
                organization: organizationPda,
                admin: testAdmin.publicKey,
            })
            .signers([testAdmin])
            .rpc();

        const organizationAccount = await program.account.organization.fetch(organizationPda);

        expect(organizationAccount.authority.equals(authority.publicKey)).to.eq(true);
        expect(organizationAccount.pendingAuthority.equals(anchor.web3.PublicKey.default)).to.eq(true);
        expect(Buffer.from(organizationAccount.organizationId).equals(organizationId)).to.eq(true);
        expect(organizationAccount.threshold).to.eq(threshold);
        expect(organizationAccount.active).to.eq(true);
        expect(organizationAccount.role).to.deep.eq(Roles.manufacturer);
        expect(organizationAccount.bump).to.eq(organizationBump);
    });

    it("deactivate organization", async () => {
        const organization = await CreateOrganization({
            program,
            admin: testAdmin,
            protocolPda
        });

        await program.methods
            .deactivateOrganization()
            .accountsPartial({
                admin: testAdmin.publicKey,
                organization: organization.organizationPda,
                protocolConfig: protocolPda,
            })
            .signers([testAdmin])
            .rpc();

        const organizationAccount = await program.account.organization.fetch(organization.organizationPda);

        expect(organizationAccount.active).to.be.eq(false);
    });
});