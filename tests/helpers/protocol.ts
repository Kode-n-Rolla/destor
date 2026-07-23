import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { Destor } from "../../target/types/destor";

export const getProtocolPda = (programid: anchor.web3.PublicKey) => {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_config")],
    programid
  );
};

export const ensureProtocolInitialized = async (
  program: Program<Destor>,
  admin: anchor.web3.Keypair
) => {
  const [protocolPda, protocolBump] = getProtocolPda(program.programId);

  const existingAccount = await program.provider.connection.getAccountInfo(
    protocolPda
  );

  if (!existingAccount) {
    await program.methods
      .initialize()
      .accountsPartial({
        admin: admin.publicKey,
        protocolConfig: protocolPda,
      })
      .signers([admin])
      .rpc();
  }

  const protocolConfig = await program.account.protocolConfig.fetch(
    protocolPda
  );

  return {
    protocolPda,
    protocolBump,
    protocolConfig,
  };
};
