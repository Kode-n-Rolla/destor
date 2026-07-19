import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../../target/types/destor";

type CreateVehicleArgs = {
    program: Program<Destor>;
    organizationPda: anchor.web3.PublicKey;
    organizationId: Buffer;
    memberPda: anchor.web3.PublicKey;
    wallet: anchor.web3.Keypair;
    vinHash?: Buffer;
    model?: string;
    color?: string;
};

export const getVehiclePda = (
    programId: anchor.web3.PublicKey,
    organizationId: Buffer,
    vinHash: Buffer
) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("vehicle"),
            organizationId,
            vinHash
        ],
        programId
    );
};

export const createVehicle = async ({
    program,
    organizationPda,
    organizationId,
    memberPda,
    wallet,
    vinHash = anchor.web3.Keypair.generate().publicKey.toBuffer(),
    model = "Pontiac GTO",
    color = "Orange"
}: CreateVehicleArgs) => {
    const [vehiclePda, vehicleBump] = getVehiclePda(
        program.programId,
        organizationId,
        vinHash
    );

    await program.methods
        .mintVehicle(
            Array.from(vinHash),
            model,
            color
        )
        .accountsPartial({
            wallet: wallet.publicKey,
            organization: organizationPda,
            member: memberPda,
            vehicle: vehiclePda
        })
        .signers([wallet])
        .rpc();

    const vehicleAccount = await program.account.vehicle.fetch(vehiclePda);

    return {
        vinHash,
        model,
        color,
        vehiclePda,
        vehicleBump,
        vehicleAccount
    };
};