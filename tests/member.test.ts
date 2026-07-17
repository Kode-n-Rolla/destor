import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../target/types/destor";
import { expect } from "chai";
import { ensureProtocolInitialized } from "./helpers/protocol";
import { testAdmin, airdrop } from "./helpers/actors";
import { createOrganization } from "./helpers/organization";
import { getMemberPda } from "./helpers/member";

describe("destor::member", () => {
    ///////////////////////////////
    //       Preparation         //
    ///////////////////////////////
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.destor as Program<Destor>;
    const airdropSolAmount = 3;

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
    it("add member for organization", async () => {
        const newMember = anchor.web3.Keypair.generate();

        const organization = await createOrganization({
            program,
            admin: testAdmin,
            protocolPda,
            authority
        });

        const [memberPda, memberBump] = getMemberPda(
            program.programId,
            organization.organizationPda,
            newMember.publicKey
        )
        
        await program.methods
            .addOrganizationMember(newMember.publicKey)
            .accountsPartial({
                authority: organization.authority.publicKey,
                member: memberPda,
                organization: organization.organizationPda
            })
            .signers([organization.authority])
            .rpc();

        const memberAccount = await program.account.member.fetch(memberPda);
        
        expect(memberAccount.organization.equals(organization.organizationPda)).to.eq(true);
        expect(memberAccount.wallet.equals(newMember.publicKey)).to.eq(true);
        expect(memberAccount.active).to.eq(true);
        expect(memberAccount.bump).to.eq(memberBump);
    });

    it("remove member from organization", async () => {
        const newMember = anchor.web3.Keypair.generate();

        const organization = await createOrganization({
            program,
            admin: testAdmin,
            protocolPda,
            authority
        });

        const [memberPda] = getMemberPda(
            program.programId,
            organization.organizationPda,
            newMember.publicKey
        )
        
        // Add member
        await program.methods
            .addOrganizationMember(newMember.publicKey)
            .accountsPartial({
                authority: organization.authority.publicKey,
                member: memberPda,
                organization: organization.organizationPda
            })
            .signers([organization.authority])
            .rpc();

        // Remove member
        await program.methods
            .removeOrganizationMember(newMember.publicKey)
            .accountsPartial({
                authority: organization.authority.publicKey,
                member: memberPda,
                organization: organization.organizationPda
            })
            .signers([organization.authority])
            .rpc();

        const memberAccountAfterRemove = await program.account.member.fetch(memberPda);

        expect(memberAccountAfterRemove.active).to.eq(false);
        expect(memberAccountAfterRemove.organization.equals(organization.organizationPda)).to.eq(true);
        expect(memberAccountAfterRemove.wallet.equals(newMember.publicKey)).to.eq(true);
    });
});