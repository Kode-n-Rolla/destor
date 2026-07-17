import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../target/types/destor";
import { expect } from "chai";
import { ensureProtocolInitialized } from "./helpers/protocol";
import { testAdmin, airdrop } from "./helpers/actors";
import { Roles } from "./helpers/roles";
import { createOrganization } from "./helpers/organization";

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

        // Action
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

        // Assertion
        expect(organizationAccount.authority.equals(authority.publicKey)).to.eq(true);
        expect(organizationAccount.pendingAuthority.equals(anchor.web3.PublicKey.default)).to.eq(true);
        expect(Buffer.from(organizationAccount.organizationId).equals(organizationId)).to.eq(true);
        expect(organizationAccount.threshold).to.eq(threshold);
        expect(organizationAccount.active).to.eq(true);
        expect(organizationAccount.role).to.deep.eq(Roles.manufacturer);
        expect(organizationAccount.bump).to.eq(organizationBump);
    });

    it("deactivate organization", async () => {
        const organization = await createOrganization({
            program,
            admin: testAdmin,
            protocolPda
        });

        // Action
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

        // Assertion
        expect(organizationAccount.active).to.be.eq(false);
    });

    it("set valid threshold for organization", async () => {
        const newThreshold = 4;

        const organization = await createOrganization({
            program,
            admin: testAdmin,
            protocolPda
        });

        const organizationBeforeChange = await program.account.organization.fetch(organization.organizationPda);

        await program.methods
            .setOrganizationThreshold(newThreshold)
            .accountsPartial({
                authority: organization.authority.publicKey,
                organization: organization.organizationPda
            })
            .signers([organization.authority])
            .rpc();

        const organizationAfterChange = await program.account.organization.fetch(organization.organizationPda);

        expect(organizationBeforeChange.threshold).to.eq(organization.threshold);
        expect(organizationAfterChange.threshold).to.be.eq(newThreshold);
    });

    it("set invalid threshold for organization", async () => {
        const newThreshold = 1;
        let failed = false;

        const organization = await createOrganization({
            program,
            admin: testAdmin,
            protocolPda
        });

        try {
            await program.methods
                .setOrganizationThreshold(newThreshold)
                .accountsPartial({
                    authority: organization.authority.publicKey,
                    organization: organization.organizationPda
                })
                .signers([organization.authority])
                .rpc();
            
            expect.fail("Expected InvalidThresholdValue error");
        } catch (err) {
            failed = true;
            const anchorError = err as anchor.AnchorError;
            expect(anchorError.error.errorCode.code).to.eq("InvalidThresholdValue");
        }
        
        const organizationAfterFail = await program.account.organization.fetch(organization.organizationPda);

        expect(failed).to.be.eq(true);
        expect(organizationAfterFail.threshold).to.eq(organization.threshold);
    });

    it("change authority", async () => {
        const newAuthority = anchor.web3.Keypair.generate();

        const organization = await createOrganization({
            program,
            admin: testAdmin,
            protocolPda
        });

        // Request authority transfer
        await program.methods
            .requestAuthorityTransfer(newAuthority.publicKey)
            .accountsPartial({
                authority: organization.authority.publicKey,
                organization: organization.organizationPda
            })
            .signers([organization.authority])
            .rpc();

        const organizationAfterRequestAuthorityTransfer = await program.account.organization.fetch(organization.organizationPda);
        expect(organizationAfterRequestAuthorityTransfer.pendingAuthority.equals(newAuthority.publicKey)).to.eq(true);

        // Accept authority transfer
        await program.methods
            .acceptAuthorityTransfer()
            .accountsPartial({
                pendingAuthority: newAuthority.publicKey,
                organization: organization.organizationPda
            })
            .signers([newAuthority])
            .rpc();

        const organizationAfterAcceptAuthorityTransfer = await program.account.organization.fetch(organization.organizationPda);
        
        expect(organizationAfterAcceptAuthorityTransfer.authority.equals(newAuthority.publicKey)).to.eq(true);
        expect(organizationAfterAcceptAuthorityTransfer.pendingAuthority.equals(anchor.web3.PublicKey.default)).to.eq(true);
        expect(organizationAfterAcceptAuthorityTransfer.authority.equals(organization.authority.publicKey)).to.eq(false);
    });
});