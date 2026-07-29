import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { Destor } from "../../target/types/destor";

type CreateVehicleArgs = {
  program: Program<Destor>;
  organizationPda: anchor.web3.PublicKey;
  memberPda: anchor.web3.PublicKey;
  wallet: anchor.web3.Keypair;
  vinHash?: Buffer;
  model?: string;
  color?: string;
};

export const getVehiclePda = (
  programId: anchor.web3.PublicKey,
  vinHash: Buffer
) => {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vehicle"), vinHash],
    programId
  );
};

export const createVehicle = async ({
  program,
  organizationPda,
  memberPda,
  wallet,
  vinHash = anchor.web3.Keypair.generate().publicKey.toBuffer(),
  model = "Pontiac GTO",
  color = "Orange",
}: CreateVehicleArgs) => {
  const [vehiclePda, vehicleBump] = getVehiclePda(program.programId, vinHash);

  await program.methods
    .mintVehicle(Array.from(vinHash), model, color)
    .accountsPartial({
      wallet: wallet.publicKey,
      organization: organizationPda,
      member: memberPda,
      vehicle: vehiclePda,
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
    vehicleAccount,
  };
};
