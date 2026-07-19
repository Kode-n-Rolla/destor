import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../target/types/destor";
import { expect } from "chai";
import { ensureProtocolInitialized } from "./helpers/protocol";
import { testAdmin, airdrop } from "./helpers/actors";
import { Roles } from "./helpers/roles";
import { createOrganization } from "./helpers/organization";
import { createMember, getMemberPda } from "./helpers/member";

describe("destor::vehicle", () => {
    ///////////////////////////////
    //       Preparation         //
    ///////////////////////////////
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.destor as Program<Destor>;

    const airdropSolAmount = 5;

    let protocolPda: anchor.web3.PublicKey;
    let organizationAuthority: anchor.web3.Keypair;
    let manufacturerWallet: anchor.web3.Keypair;
    let organization: Awaited<ReturnType<typeof createOrganization>>;
    let member: Awaited<ReturnType<typeof createMember>>;

    ///////////////////////////////
    //           SetUp           //
    ///////////////////////////////
    before(async () => {
        await airdrop(provider, testAdmin.publicKey, airdropSolAmount);

        const protocol = await ensureProtocolInitialized(program, testAdmin);

        protocolPda = protocol.protocolPda;
    });

    beforeEach(async () => {
        organizationAuthority = anchor.web3.Keypair.generate();
        manufacturerWallet = anchor.web3.Keypair.generate();

        await airdrop(provider, organizationAuthority.publicKey, airdropSolAmount);
        await airdrop(provider, manufacturerWallet.publicKey, airdropSolAmount);

        organization = await createOrganization({
            program,
            admin: testAdmin,
            protocolPda,
            authority: organizationAuthority,
            role: Roles.manufacturer
        })
    });

    ///////////////////////////////
    //     Happy Path Tests      //
    ///////////////////////////////
    it("mint vehicle account", async () => {

    });

    it("assign initial owner", async () => {

    });

    it("success vehicle transfer", async () => {

    });


});