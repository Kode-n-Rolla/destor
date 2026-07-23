import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { Destor } from "../../target/types/destor";
import { Roles } from "./roles";

type CreateOrganizationArgs = {
  program: Program<Destor>;
  admin: anchor.web3.Keypair;
  protocolPda: anchor.web3.PublicKey;
  authority?: anchor.web3.Keypair;
  organizationId?: Buffer;
  role?: any;
  threshold?: number;
};

export const getOrganizationPda = (
  programId: anchor.web3.PublicKey,
  organizationId: Buffer
) => {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("organization"), organizationId],
    programId
  );
};

export const createOrganization = async ({
  program,
  admin,
  protocolPda,
  authority = anchor.web3.Keypair.generate(),
  organizationId = anchor.web3.Keypair.generate().publicKey.toBuffer(),
  role = Roles.manufacturer,
  threshold = 2,
}: CreateOrganizationArgs) => {
  const [organizationPda, organizationBump] = getOrganizationPda(
    program.programId,
    organizationId
  );

  await program.methods
    .registerOrganization(
      role,
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

  const organizationAccount = await program.account.organization.fetch(
    organizationPda
  );

  return {
    authority,
    organizationId,
    organizationPda,
    organizationBump,
    organizationAccount,
    role,
    threshold,
  };
};
