import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../target/types/destor";
import { expect } from "chai";
import { ensureProtocolInitialized } from "./helpers/protocol";
import { testAdmin, airdrop } from "./helpers/actors";
import { Roles } from "./helpers/roles";
import { createOrganization } from "./helpers/organization";
import { createMember } from "./helpers/member";
import { createVehicle, getVehiclePda } from "./helpers/vehicle";

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
        });

        member = await createMember({
            program,
            organizationPda: organization.organizationPda,
            authority: organization.authority,
            wallet: manufacturerWallet
        });
    });

    ///////////////////////////////
    //     Happy Path Tests      //
    ///////////////////////////////
    it("mint vehicle account", async () => {
        const vinHash = anchor.web3.Keypair.generate().publicKey.toBuffer();
        const model = "Chrysler 300C";
        const color = "Black";

        const [vehiclePda, vehicleBump] = getVehiclePda(
            program.programId,
            organization.organizationId,
            vinHash
        )

        await program.methods
            .mintVehicle(
                Array.from(vinHash),
                model,
                color,
            )
            .accountsPartial({
                wallet: manufacturerWallet.publicKey,
                organization: organization.organizationPda,
                member: member.memberPda,
                vehicle: vehiclePda
            })
            .signers([manufacturerWallet])
            .rpc();

        const vehicleAccount = await program.account.vehicle.fetch(vehiclePda);

        expect(Buffer.from(vehicleAccount.vinHash).equals(vinHash)).to.eq(true);
        expect(vehicleAccount.nftAsset.equals(anchor.web3.PublicKey.default)).to.eq(true);
        expect(vehicleAccount.manufacturer.equals(organization.organizationPda)).to.eq(true);
        expect(vehicleAccount.owner.equals(anchor.web3.PublicKey.default)).to.eq(true); // @todo when NFT exist, dont forget
        expect(vehicleAccount.model).to.eq(model);
        expect(vehicleAccount.color).to.eq(color);
        expect(vehicleAccount.mileage.toNumber()).to.eq(0);
        expect(vehicleAccount.noteCount.toNumber()).to.eq(0);
        expect(vehicleAccount.ownerCount).to.eq(0);
        expect(vehicleAccount.bump).to.eq(vehicleBump);
        expect(vehicleAccount.manufacturedAt.toNumber()).to.be.greaterThan(0);
    });

    it("assign initial owner", async () => {
        const owner = anchor.web3.Keypair.generate();

        const vehicle = await createVehicle({
            program,
            organizationPda: organization.organizationPda,
            organizationId: organization.organizationId,
            memberPda: member.memberPda,
            wallet: manufacturerWallet
        });

        await program.methods
            .assignInitialOwner(
                Array.from(vehicle.vinHash),
                owner.publicKey
            )
            .accountsPartial({
                wallet: manufacturerWallet.publicKey,
                vehicle: vehicle.vehiclePda,
                organization: organization.organizationPda,
                member: member.memberPda
            })
            .signers([manufacturerWallet])
            .rpc();

        const vehicleAccount = await program.account.vehicle.fetch(vehicle.vehiclePda);

        expect(vehicleAccount.owner.equals(owner.publicKey)).to.eq(true);
        expect(vehicleAccount.ownerCount).to.eq(1);
        expect(Buffer.from(vehicleAccount.vinHash).equals(vehicle.vinHash)).to.eq(true);
        expect(vehicleAccount.manufacturer.equals(organization.organizationPda)).to.eq(true);
    });

    it("success vehicle transfer", async () => {
        let vehicle: Awaited<ReturnType<typeof createVehicle>>;

        const owner = anchor.web3.Keypair.generate();
        const newOwner = anchor.web3.Keypair.generate();

        await airdrop(provider, owner.publicKey, airdropSolAmount);

        vehicle = await createVehicle({
            program,
            organizationPda: organization.organizationPda,
            organizationId: organization.organizationId,
            memberPda: member.memberPda,
            wallet: manufacturerWallet
        });

        await program.methods
            .assignInitialOwner(
                Array.from(vehicle.vinHash),
                owner.publicKey
            )
            .accountsPartial({
                wallet: manufacturerWallet.publicKey,
                vehicle: vehicle.vehiclePda,
                organization: organization.organizationPda,
                member: member.memberPda
            })
            .signers([manufacturerWallet])
            .rpc();

        const vehicleAccountBeforeTransfer = await program.account.vehicle.fetch(vehicle.vehiclePda);

        await program.methods
            .transferVehicle(
                Array.from(vehicle.vinHash),
                newOwner.publicKey,
            )
            .accountsPartial({
                seller: owner.publicKey,
                vehicle: vehicle.vehiclePda,
                organization: organization.organizationPda
            })
            .signers([owner])
            .rpc();

        const vehicleAccountAfterTransfer = await program.account.vehicle.fetch(vehicle.vehiclePda);

        expect(vehicleAccountBeforeTransfer.owner.equals(owner.publicKey)).to.eq(true);
        expect(vehicleAccountAfterTransfer.owner.equals(newOwner.publicKey)).to.eq(true);

        expect(vehicleAccountBeforeTransfer.ownerCount).to.eq(1);
        expect(vehicleAccountAfterTransfer.ownerCount).to.eq(2);

        expect(Buffer.from(vehicleAccountAfterTransfer.vinHash).equals(vehicle.vinHash)).to.eq(true);
        expect(vehicleAccountAfterTransfer.manufacturer.equals(organization.organizationPda)).to.eq(true);
        expect(vehicleAccountAfterTransfer.model).to.eq(vehicle.model);
        expect(vehicleAccountAfterTransfer.color).to.eq(vehicle.color);
    });
});